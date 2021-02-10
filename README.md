# RRD Treadmill
 Modify RRD step values
 
 This is a Rust rewrite of the Python version at https://github.com/AndrewPiroli/rrd-step-changer
 It's way faster!


This bad boy takes RRD files that have been dumped to XML, installs a new step and heartbeat in there, and duplicates the records enough times to keep the timescale accurate.

Use at your own risk!

## Usage

Step 0) Back up your RRD
  `cp -a /path/to/file.rrd ./file.rrd.backup`

Step 1) Dump RRD
  `rrdtool dump /path/to/file.rrd > original.xml`
  
Step 2) Change RRD Step (example: step 60, heartbeat 120)
  `rrd_treadmill original.xml  modified.xml 60 120`

Step 3) Restore RRD
  `rrdtool restore modified.xml modified.rrd`
  
Step 4) Copy back
    `cp modified.rrd /path/to/file.rrd`


## Limitations

~~It only can decrease step. You can't make it longer.~~

Now possible to increase step. (Beta level feature)

&nbsp;

It can only change the step to a numerical factor of the existing step. 

300 <-> 60 :ok_hand: 

300 <-> 100 :ok_hand:

300 <-> 200 :-1:



&nbsp;

It doesn't do any interpolations or anything smart with the records, just duplicates/skips rows until it fills in the time properly.
