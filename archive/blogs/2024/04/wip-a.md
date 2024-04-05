# The Exploding Story Code Overview

Tagged story: [This Story Did Not Explode](https://www.fimfiction.net/story/553695/this-story-did-not-explode)

***

## Prelude
Before I start the code overview, I wanted to say thanks to everypony who commented on or read the story. It was really fun to work on with [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus), we both really appreciate the response the story has gotten.

I should also explain what happened, in case you weren't there for the event.

On April 1st, 2024, PseudoBob released a story with a simple title: This Story will Explode in 24:00. And every minute the title counted down, 23:59 → 23:58 → 23:57, etc.

On every hour, a new chapter was released, and the cover was updated. The description and short description were also updated regularly. If you paid close attention, you might have even found some Easter eggs.  
![:raritywink:](../../../emotes/raritywink.png)

Last thing before we get into the code talk, if you want to read more about Bob's side of the story, check out his sister blog to this one [here](). And if you have questions, feel free to ask either of us questions [here]() in our Q&A forum post.

## Code Overview

The code has four mane packages to make it work:
1. [ FIMFiction Cookie](https://github.com/SilkRose/Pony/tree/mane/archive/code/typescript/fimfic-cookie): to get a cookie for use with the cover updater.
2. [FIMFiction Cover](https://github.com/SilkRose/Pony/tree/mane/archive/code/typescript/fimfic-cover): to update the story's cover.
3. [Clock Timer 2](https://github.com/meadowsys/wiwi/blob/wiwi/src/clock_timer_2.rs) ([wiwi](https://github.com/meadowsys/wiwi)): to keep all the events running at the right time.
4. [FIMFiction April Fools](https://github.com/SilkRose/Pony/tree/mane/archive/code/rust/fimfic-april-fools-2024): to bring it all together and send out API requests.

We will go over these one at a time.

### FIMFiction Cookie

This one is really straightforward, it uses a browser emulator to log in to your FIMFiction account and saves a `cookies.json` file in the root of the project directory.

There are no command line arguments, it asks for your username or email, and password in the console.

**Please note:** the `cookies.json` file should be kept secret, for it can be used to log in to your account.

This package was written in Typescript and is 59 lines of code.

### FIMFiction Cover

This one is also pretty straightforward, but a little more complex. It uses the `cookies.json` file and opens a page in a browser emulator, going to the stories manage page before hitting the `Browse...` button, then entering the file path of the new cover. After that, it clicks the `Save Changes` button, and waits for the page to refresh before closing.

It has the following command line arguments:
1. Story ID.
2. File path to the new cover.
3. File path to the `cookies.json` file.

This package was written in Typescript and is 92 lines of code.

### Clock Timer 2 (wiwi)

> Internally, a clock timer stores a time (ex. 1 Apr 2024 at midnight UTC), and an interval amount (ex. 1 minute), and exposes a way to get the next time in the interval. Every time it's called, it adds the interval amount to the stored time (ex. 00:00 + 1 minute = 00:01), and checks if the current time is before this new calculated time. If it's already past, then it returns to this new time without waiting. Otherwise, it'll wait until he calculated time is reached, then returns it. What this achieves is a way to halt the program until certain time intervals. The program can then do things with this time information, like call FIMFiction API every minute to update the title.
>
> ~ *Meadowsys*

This module is a part of the crate wiwi, was written in Rust, and is 186 lines of code.

### FIMFiction April Fools

This part is where I will go into greater detail, since this is the mane part of the whole event.

Let's start with the command line arguments:
1. API Access Token.
2. Path to `arguments.json` file.
3. Path to `events.json` file.

#### API Access Token

This is required to make any call to the [FIMFiction API](https://www.fimfiction.net/developers/api/v2/docs). To get a token, you must authenticate with an application on the API. Getting an application requires knighty's approval, and can take time.

**Please note:** your access token should be kept secret, for it can be used to do stuff on your account, and they never expire.

#### Arguments.json

This file contains the mane arguments so that the command wasn't so long.

This file contains the following data:
1. Story ID: number
   - Self explanatory. ID can be found in the URL of your story.
2. Start time: number
   - The Unix Epoch time of when you want it to start the event.
3. Skip past events: boolean
   - Whether to skip past events, like if the program was interrupted.
4. Duration hours: number
   - The number of hours the entire event lasts.
5. Interval minutes: number
   - How often in minutes the clock ticks.
6. Countdown duration hours: number
   - The hours to countdown in the title.
7. Covers directory: string
   - where all the cover files are stored.
8. Cover mane.js: string
   - The mane script file for FIMFiction Cover.
9. `cookies.json` path: string
   - The `cookies.json` file path gotten from FIMFiction Cookie.

#### Events.json



This application was written in Rust and is 254 lines of code.
