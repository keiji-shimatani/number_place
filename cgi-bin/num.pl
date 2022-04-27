#!/usr/bin/env -S perl -I.

use CGI qw/:standard *table *p/;
use Fcntl qw(:DEFAULT :flock);

#use constant {
#    QUESTION => 1,	# page status
#    ANSWER   => 2,	# page status
#    ERROR    => 3,	# page status
#    INITIAL   => 1,	# search status
#    CONTINUED => 2,	# search status
#    MAIL_ADD => 'xxxxxxxx@gmail.com',
#    EXE_FILE => '/home/xxxxxxxx/number_place/number_place',
#    LOG_FILE => '/home/xxxxxxxx/number_place/number_place_log.txt',
#    COUNT_FILE => './count',
#    SINCE_DATE => '2007-04-16',
#    LOCK_FILE => './count.lock',
#    LOCK_DISCARD_MINUTE => 60,
#    LOCK_RETRY => 5,
#};
$QUESTION = 1;	# page status
$ANSWER   = 2;	# page status
$ERROR    = 3;	# page status
$INITIAL   = 1;	# search status
$CONTINUED = 2;	# search status
$MAIL_ADD => 'xxxxxxxx@gmail.com',
$EXE_FILE = '/home/xxxxxxxx/number_place/number_place';
$LOG_FILE = '/home/xxxxxxxx/number_place/number_place_log.txt';
$COUNT_FILE = './count';
$SINCE_DATE = '2022-04-27';
$LOCK_FILE = './count.lock';
$LOCK_DISCARD_MINUTE = 60;
$LOCK_RETRY = 5;

sub log_out {
    return;
    open(_LOG, '>> '.$LOG_FILE);
    print _LOG join("\n", @_);
    print _LOG "\n";
    close _LOG;
}

sub isValidRow
{
    my $str = shift;
    return $str =~ /^[0-9]{9}$/
}

sub isValidSequence
{
    my $str = shift;
    return $str =~ /^[0-9]{81}$/
}

sub isValidCells
{
    my $str = shift;
    return $str =~ /^[0-9]{81}$/
}

sub isValidAnswer
{
    my $str = shift;
    return $str =~ /^[1-9]{81}$/
}

sub printBoard
{
    my $html = '';
    my $cells = shift;
    my $caption = shift;

#    log_out("cells:${cells}|");
#    $cells = "12345678";
    $cells .= "000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    my @cells = split(//, substr($cells, 0, 81));
    @cells = map { $_ || '_' } @cells;
    
#    print start_table({-border => 1, cellpadding => 2});
    print start_table({-border => 1});
    if ($caption) {
	print caption($caption);
    }
    for (my $i = 0; $i < 9; $i++) {
	my @cols = map { td($_) } splice(@cells, 0, 9);
	print Tr({-align => CENTER}, @cols);
    }
    print end_table;

}

sub unlock
{
    flock(LOCK_FH, LOCK_UN);
}

sub lock
{
    for (my $i = 0; $i < $LOCK_RETRY; $i++) {
	if (flock(LOCK_FH, LOCK_EX)) {
	    return 1;
	}
	sleep(1);
    }
    return 0;
}

sub getCount
{
    if (!sysopen(LOCK_FH, $COUNT_FILE, O_RDWR | O_CREAT)) {
	return undef;
    }
    if (!lock()) {
	close(LOCK_FH);
	return undef;
    }
    my $count = <LOCK_FH> || 0;
    seek(LOCK_FH, 0, 0);
    truncate(LOCK_FH, 0);
    $count++;
    print LOCK_FH $count, "\n";
    unlock;
    close LOCK_FH;
    return $count;

#    if (!open(LOCK_FH, '>', $LOCK_FILE)) {
#	return undef;
#    }
#    if (!lock()) {
#	close(LOCK_FH);
#	return undef;
#    }
#    if (!open(COUNT_FH, '<', $COUNT_FILE)) {
#	unlock();
#	close(LOCK_FH);
#	return undef;
#    }
#    my $count = <COUNT_FH>;
#    close(COUNT_FH);
#    if (open(COUNT_FH, '+>', $COUNT_FILE)) {
#	$count++;
#	print COUNT_FH $count;
#	close(COUNT_FH);
#    }
#    unlock();
#    close(LOCK_FH);
#    return $count;
}

my $pageStatus = $QUESTION;
my $searchStatus = $INITIAL;
my $cells, $sequence;
$cells = $sequence = '';
if (param()) {
    $pageStatus = $ANSWER;
    $cells = param('cells');
    if ($cells) {
	if (!isValidCells($cells)) {
	    $pageStatus = $ERROR;
	}
    } else {
	$cells = '';
	for (my $i = 0; $i < 9; $i++) {
	    my $row = param("row$i");
	    if (!isValidRow($row)) {
		$pageStatus = $ERROR;
		last;
	    }
	    $cells .= $row;
	}
    }
    $sequence = param('sequence');
    if ($sequence) {
	if (!isValidSequence($sequence)) {
	    $pageStatus = $ERROR;
	} else {
	    $searchStatus = $CONTINUED;
	}
    } else {
	$sequence = '';
    }
}

my $answer;
my $pid;
if ($pageStatus == $ANSWER) {
    $pid = open(FH, $EXE_FILE." $cells $sequence |");
    if (defined $pid) {
	$answer = <FH>;
	chomp $answer;
	$sequence = <FH>;
	chomp $sequence;
    } else {
	$pageStatus = $ERROR;
    }
    close FH;
}

Delete_all();
print header(-expires => 'now'),
      start_html('Number Place'),
      h1({-align => center}, 'Number Place');
if ($pageStatus == $QUESTION) {
    my $count = getCount();
    print start_p({-align => center});
    print "Total access: $count", br,
	  "since ".$SINCE_DATE, br, br,
	  "Input question.", br,
	  "Use 0 as void cell.";
    print start_form;
    for (my $i = 0; $i < 9; $i++) {
	print textfield(-name => "row$i",
			-size => 9,
			-maxlength => 9), br;
    }
    print submit(-name=>'Solve'),
	  end_form;
    print end_p;
} elsif ($pageStatus == $ANSWER) {
#    log_out("cells:${cells}|");
    print start_p({-align => center});
    printBoard($cells, "Question");
    if (isValidAnswer($answer)) {
	print br;
	printBoard($answer, "Answer");
	print start_form(-action => url),
	      hidden('cells', $cells),
	      hidden('sequence', $sequence),
	      submit(-name => 'Next'),
	      end_form;
    } else {
	if ($searchStatus == $INITIAL) {
	    print br, "There is no answer.", br, br;
	} else {
	    print br, "There is no more answer.", br, br;
	}
    }	
    print a({href => url}, 'Top');
    print end_p;
} else { # Error
    print start_p({-align => center}),
	  'Error!', br, br,
	  a({href => url}, 'Top'),
	  end_p;
}

print start_p({-align => center}),
      a({href => 'mailto:'.$MAIL_ADD}, Mail),
      end_p,
      end_html;
