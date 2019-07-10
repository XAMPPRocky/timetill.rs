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

## What events belong on `timetill.rs`.
Timetill.rs is focused on bigger Rust events. Regular meet-ups should be posted
on the [community calendar]. If you're unsure if your event is big enough think
_"Would someone travel from outside the city to specifically come to your
event?"_, if the answer is yes then you should add it!

All events that are featured on timetill.rs are required to have a Code of
Conduct that is available to read on the website. The CoC should also be along
the same lines as [The Rust Programming Language CoC], i.e. joke CoC's or CoC's
that encourage harm or discrimination will be not be accepted.

## Event Schema
```json
{
    // This should be the name of the conference
    "name": "RustConf",
    // This should be the start date of the conference.
    "date": "2099-01-01T09:00:00-05:00",
    // This should be a short summary  about your conference and what yours
    // unique.
    "blurb": "RustConf gathers Rust developers from around the world to learn and share with one another. It kicks off with optional training courses which lead into the Day 2 schedule of keynote speeches, technical talks and a community happy hour.",
    // This should be where your conference is located. This should be in the
    // format of "<CITY>, <COUNTRY>", please do not include the state or county.
    "location": "Portland, USA",
    // The website for your conference.
    "website": "//rustconf.com/",
    // The link to the schedule of the conference.
    "schedule": "//rustconf.com/schedule",
    // This is a simple description list. The features should try to include as
    // much of the features listed below as possible. More can be added if you
    // think a feature of your conference really needs to stand out.
    "features": {
        // How long the conference is.
        "duration": "2 days",
        // What kind of conference is it, i.e. a conference or a hackfest.
        "kind": "conference",
        // This should be standard ticket price, not the discounted price.
        "price": "$299",
        // If your conference has a RustBridge workshop.
        "RustBridge": "yes",
        // If your conference has workshops.
        "workshops": "yes",
        // If your conference has live transcription services which help make
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
All design feedback should be provided as an issue. Design isn't like
programming, everyone has different tastes so arbitrary concepts like changing
the colours based on your preference will likely be closed. This is done for the
sake of maintainability. This is a project that is being done in my spare
time for no money.

Design feedback is always appreciated though, especially in regards to improving
accessibility of website.

# Tech
The website is built using Vue.js, Bootstrap, Sass, and Parcel. Requests for
changing to different frameworks will be rejected. The current website footprint
is around ~265kB. This could be reduced but would have little gains for amount
work required.

[`node`]: //nodejs.org/
[`yarn`]: //yarnpkg.com
[`parcel`]: //parceljs.org
[`conferences.json`]: ./conferences.json
[community calendar]: //www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[The Rust Programming Language CoC]: https://www.rust-lang.org/policies/code-of-conduct
