# Contributing

# Building the website
To build the website you need to have [`node`], [`yarn`], and [`parcel`]
installed.

To build and run the website run the following command.
```
parcel index.html
```

# Adding your event
To add your event simply make a pull request adding it to the
[`conferences.json`] file.

## Does my event belong on `timetill.rs`?
Timetill.rs is focused on bigger Rust events. Regular meet-ups should be posted
on the [community calendar]. If you're unsure if your event is big enough ask
yourself _"Would attendees travel from outside the local area to come to your
event?"_, if the answer is yes then it belongs!

All events that are featured on timetill.rs are required to have and enforce a
Code of Conduct that is available to read on their website.

## Event Schema
```javascript
{
    // The name of your conference.
    "name": "RustConf",
    // The start date of your conference.
    "date": "2099-01-01T09:00:00-05:00",
    // A short summary  about your conference and why someone should come to
    // your conference.
    "blurb": "RustConf gathers Rust developers from around the world to learn
        and share with one another. It kicks off with optional training courses
        which lead into the Day 2 schedule of keynote speeches, technical talks
        and a community happy hour.",
    // Where your conference is located. This should be in the
    // format of "<CITY>, <COUNTRY>".
    "location": "Portland, USA",
    // The link to your website for your conference.
    "website": "//rustconf.com/",
    // The link to the schedule of the conference.
    "schedule": "//rustconf.com/schedule",
    // A description list of conference features. If you do not have some of the
    // features mentioned, please delete the entry. Feel free to also include
    // features not mentioned.
    "features": {
        // Total duration of the conference.
        "duration": "2 days",
        // What kind of conference is it, i.e. a conference or a hackfest.
        "kind": "conference",
        // This should be a range of prices or the standard ticket price.
        "price": "$299",
        // Does your conference has a RustBridge workshop.
        "RustBridge": "yes",
        // If your conference has workshops.
        "workshops": "yes",
        // If your conference has live transcription services that help make
        // your conference more accessible.
        "live transcription": "yes",
        // If your conference is wheelchair accessible.
        "wheelchair accessible": "yes",
        // If your conference provides childcare services to parents who need to
        // bring children.
        "childcare": "yes"
    }
},
```

# Design
All design feedback should be provided as an issue on GitHub first, before
providing a PR.

# Code
The website is built using Vue.js, Bootstrap, Sass, and Parcel. The current
website footprint is around ~265kB.

[`node`]: //nodejs.org/
[`yarn`]: //yarnpkg.com
[`parcel`]: //parceljs.org
[`conferences.json`]: ./conferences.json
[community calendar]: //www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[The Rust Programming Language CoC]: https://www.rust-lang.org/policies/code-of-conduct

