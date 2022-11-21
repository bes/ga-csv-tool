# Number of users per OS major version

This tool currently does _one_ thing — count the number of users per *major* OS version.
To do that you need to prepare some things.

## Pull your data from Google Analytics

Go to Google Analytics and export your user data as a CSV. At the time of writing you can do that
by going to your project, then Reports > User > Tech > Tech details: OS version, then press the
"Share this report" icon somewhere in the top right corner. Then select Download File > Download CSV.

## Clean your data

Open the file you downloaded, called `data-export.csv` (by default) and clean it like this:

* Remove everything from the top down to the `# All Users` section.
* Remove all the comment `#` headers in the `# All Users` section, including `# All Users` comment.
* Leave the `OS version,Users,New users,Engaged sessions,Engagement rate,Engaged sessions per user,Average engagement time,Event count,Conversions,Total revenue` header in place.
* The header row should be on the first line.
* Make sure that the rows below the header are all data rows.
* After the data rows there should be nothing more.

## Run this application

```shell
cargo run -q -- < /path/to/cleaned-data-export.csv
```

## Result

It will output the stats per major version of the OS for the data in the CSV.

```shell
Version   Users   Pct.   New users Sessions
16.0.0    90      90.0%  90        490
15.0.0    10      10.0%  10        128
```
