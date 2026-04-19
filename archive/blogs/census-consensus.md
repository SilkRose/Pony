# Census Consensus

Tagged story: [Census Consensus](https://www.fimfiction.net/story/589323/census-consensus)

***

## Introduction

With the release of this blog post, our [custom survey site](https://census.silkrose.dev/) has been updated to allow anyone to vote on all the surveys! It lets you view the current results for each chapter, and a random results page to see any combination of answers. If you missed the event, please go vote and make your voice heard!

In addition to allowing you to vote for missed surveys, you can also vote more than once to re-cast your ballot. We've also made the behind the scenes pages visible to the public. You can see exactly how chapters and questions were written and see every revision of them.

This story was a lot of fun, but it was also a lot of stress. The idea was chosen in early December, when I was naive and thought it'd only take a month to a month and a half to code everything. In mid-December I made a group on Discord with myself and nine other people interested in the project. [Meadowsys](https://www.fimfiction.net/user/487213/meadowsys) started on the code at the end of December, and I joined her in coding in the middle of January. We ended up coding right up to and including April 1st. We even had to re-deploy twice to fix bugs while the event was happening. But, we'll talk about that later.

## Collaborators

Before I continue, I'd like to thank everypony below who helped with this project throughout it's development:

- [ashley1227](https://www.fimfiction.net/user/499793/ashley1227)
- [FanOfMostEverything](https://www.fimfiction.net/user/1400/FanOfMostEverything)
- [hawthornbunny](https://www.fimfiction.net/user/77473/hawthornbunny)
- [heaviside__](https://www.fimfiction.net/user/860080/heaviside__)
- [Hipponous](https://www.fimfiction.net/user/875988/Hipponous)
- [Lunaria](https://www.fimfiction.net/user/68640/Lunaria)
- [Math Spook](https://www.fimfiction.net/user/612387/Math+Spook)
- [meadowsys](https://www.fimfiction.net/user/487213/meadowsys)
- [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus)
- [Rego](https://www.fimfiction.net/user/180061/Rego)
- [RunicTreetops](https://www.fimfiction.net/user/489485/RunicTreetops)
- [Scriblits Talo](https://www.fimfiction.net/user/495925/Scriblits+Talo)
- [Shakespearicles](https://www.fimfiction.net/user/83757/Shakespearicles)
- [Shay492](https://www.fimfiction.net/user/840747/Shay492)
- [Silver Needle](https://www.fimfiction.net/user/463467/Silver+Needle)

I guess I should explain how the event worked before going into more detail.

Once the story was live, a custom survey site allowed users to vote on a survey that would affect the outcome of a in-universe census that the Mane 6 were holding. Everything from the questions, options, and results were all written before the event went live, so the code could publish the results for whatever options won.

If that didn't explain it well enough, you can read [Math Spook's](https://www.fimfiction.net/user/612387/Math+Spook) blog post: [Behind the scenes of “Census Consensus”](https://www.fimfiction.net/blog/1138770/behind-the-scenes-of-census-consensus), it does a great job of explaining the event. You should read it even if you understood my explanation, it's a good read and he's a great writer.

Let's start at the beginning. Late last year [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus) told me he wasn't planning to do an April Fools event this year. I'd helped him do two of these in the past, so I felt like I could step up and do something instead. I mean how hard could it be? The last two years, while hectic, weren't *that* bad…

## Previous Events

### The Exploding Story

Now, while in the middle of explaining this story, let's go back even farther to the first April Fools event I helped with: [This Story Did Not Explode](https://www.fimfiction.net/story/553695/this-story-did-not-explode), or as we called it internally: The Exploding Story. I was the one who came up with the idea. We both immediately liked the idea and went forward with it. [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus) wrote it while I coded it with help from [meadowsys](https://www.fimfiction.net/user/487213/meadowsys) for the timing code. I also wrote two chapters. Had to insert romance into it somehow, right?

This story exploded onto the scene, immediately getting attention. The event was awesome and ran without a hitch. Well, at least my code went off without a hitch, you can read more about this event in Bob's blog on it: [The Real Explosion Was The Friends We Made Along The Way](https://www.fimfiction.net/blog/1036675/the-real-explosion-was-the-friends-we-made-along-the-way). I also wrote a blog post about the code.  You can read it here: [The Exploding Story Code Overview](https://www.fimfiction.net/blog/1036674/the-exploding-story-code-overview).

A fun fact about this story: while it was live, an archival friend of mine messaged me to ask if was really going to explode, because he wanted to know if he had to stay up all night to archive the story so it wouldn't be lost.

I bring up The Exploding Story to say: this is where it all began. I won't speak for Bob, but for me this is where I realized a story could be more than just a story, it could be an event. We immediately knew that whatever we did next year, we wanted it to be interactive. This was something we'd always wanted, but didn't have the time for this first event.

### The Democracy Story

Next year's event, was exactly that, interactive. [Democracy Manifest](https://www.fimfiction.net/story/575601/democracy-manifest), known as The Democracy Story, was interactive; it used the like button as a way to vote on proposals in the story. I wrote the code while Bob did the writing, but this time with some outside help. I also wrote two chapters.

Something interesting about the code that only I utilized was: it allowed branching paths. I wrote the chapter where you vote on if Pinkie or Fluttershy are cuter. Then whoever is voted less cute, goes back to the voting ponies to ask them if she should ask out the other pony. If you'd like to read the alternative chapters, you can do so [here](https://github.com/SilkRose/Pony/blob/mane/stories/democracy-manifest/democracy-manifest-meta.md) in my [Pony](https://github.com/SilkRose/Pony) repository.

A fun fact about this story: no matter which way you vote on my two chapters, I made it so they always ended up together.

I recently re-read this story, and I really liked this line I wrote in it:
> Fluttershy smiles, appearing more relaxed after getting that off her chest. "She's just so amazing, I really want to ask her out, but I don't know if I could do it without the support of a bunch of ponies I don't know who agree that she is cuter than me."

You might have noticed that neither Bob or I wrote a blog about this story. Bob was burnt out from writing, and was a little disappointed with it, so he didn't want to write a blog. I didn't want to write one unless Bob was going to, and the code was messier and would be harder to explain.

This story had what can only be described as an obvious flaw in its design: you can only like a story once. With the fact removing a like counted as a no vote, this made interacting with the mechanics of the story very clunky. There was a five minute window between chapters you could remove a like to get your vote back, but I don't know how many people realized this.

Some readers interpreted that disliking the story counted as a no vote. We tried our best to explain that only the like button was used. I think this misconception is why the story has so many dislikes.

If I had to rank these two stories in terns of fun and enjoyment, I'd put The Exploding Story above The Democracy Story. While they were both fun, The Democracy Story didn't have that explosive energy of the first one.

I'm not sorry for any of the explosion puns.

## Early Development

Now, let's get back to this year's event: [Census Consensus](https://www.fimfiction.net/story/589323/census-consensus).

For the last 2 years, I had someone to call the shots for me. Bob was an amazing coordinator for these April Fools events. This was the first event where I had to lead the ship myself.

I wanted this whole project to be as collaborative as I could get from the very start. I didn't want to arbitrarily decide anything unless it was absolutely necessary. This is why the Discord group was made before any code had even been written.

The first thing I coded for the project was the database SQL, over a thousand lines of create, delete, update, and insert SQL statements. Once I got the table creation done, [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus) helped me with fixing the table designs. He says he didn't help with this event, but he did help with this in the very beginning.

The original plan for the code was for myself to code the back end server stuff, while [meadowsys](https://www.fimfiction.net/user/487213/meadowsys) coded the front end using a specific library called [Leptos](https://www.leptos.dev/). Unfortunately, we never got to the point of converting my back end HTML template code to the new system, as we ran out of time. And the code I had written on the back end using an HTML template library called [Maud](https://maud.lambda.xyz/) was functioning well. Yes, that library's name is a MLP reference.

A fun fact about this story: [hawthornbunny](https://www.fimfiction.net/user/77473/hawthornbunny) thought of a question idea early on about a pony working in the factory the census was being printed in getting stuck, and asking a question on the survey as a way to call for help. We never found a way to use this, but we all liked the idea.

By the time the database code was done, meadow had authentication working and I could start working on the pages for writers to create and edit questions and chapters.

## Site Screenshots

### Early Site Pages

Here is an early screenshot of the `/chapters` page:  
![Chapter's page early look](./census-consensus-images/01.png)

The add button orders the chapter. Only ordered chapters get posted when the event is live. The up and down arrows for vote duration adjust the time for voting on that chapter's survey.

Here is an early screenshot of the `/chapters/{id}/revisions` page:  
![Chapter revision page early look](./census-consensus-images/02.png)

This shows every revision of the chapter, so no data is lost.

Here is an early screenshot of the `/questions/new` page:  
![New question page early look](./census-consensus-images/03.png)

This page was later moved to be at the bottom of the question list page. Response percentage is how many ponies in-universe responded. If this is set to 50% then the answers to the survey are scaled so the total count is half of 50,240,000.

Here is an early screenshot of the `/questions/{id}/revisions` page:  
![Question revision page early look](./census-consensus-images/04.png)

Here is an early screenshot of the `/chapters/{id}/questions` page:  
![Chapter questions page early look](./census-consensus-images/05.png)

This page lets you add a question to a chapter and move around the order within that chapter. It also lets you claim a question, signalling that you plan to write that question.

It took a while, but I eventually got all the pages working. Initially I made the questions and chapters pages use HTML tables, but eventually this changed as you will see in a bit.

### Color Schemes

The next thing I worked on was the color schemes. As you might have seen on the site, the color themes are Celestia for light and Luna for dark. I used the [MLP-VectorClub](https://mlpvector.club/) website and the [Realtime Colors](https://www.realtimecolors.com/) website to create the themes. You can view the original versions here: [Celestia](https://www.realtimecolors.com/?colors=5e2f79-fef6fb-fcd8b6-fdf5b4-f2d9e8&fonts=Inter-Inter), [Luna](https://www.realtimecolors.com/?colors=a7bef1-171a35-3adfc3-00c5cc-aba4f4&fonts=Inter-Inter).

After the site was functional and looked decent, it was time to start writing. The first question of the story was created on March 3rd 2026 at 5:39AM UTC. Yes, the entire story was written very late into development. Some things never change.

I used the Luna theme initially, as I love dark mode everything, but something about the Celestia theme made me switch, and I've been using it ever since. A few people mentioned in their feedback that they really liked the themes being Celestia and Luna. Thank you!

### Event Complete Pages

Here is a screenshot of the `/user` page for an admin:  
![Admin user page](./census-consensus-images/06.png)

It had an extra spot to update a user's role, and a spot to ban a user. We wanted to be prepared for anything. Luckily, we never had to ban anypony!

Here is the `/chapters` page:  
![Chapters page](./census-consensus-images/07.png)

This shows all the relevant information while still looking good on desktop and mobile.

Here is the mobile `/chapters` page:  
![Chapters page](./census-consensus-images/07-mobile.jpg)

Here is the bottom of the `/chapters` page:  
![Chapters page bottom](./census-consensus-images/08.png)

This has the new chapter form.

Here is the `/chapters/{id}/revisions` page:  
![](./census-consensus-images/09.png)

It shows every revision in an HTML details element, including the date and time it was saved and who made the revision.

Here is the `/questions` page:  
![](./census-consensus-images/10.png)

This shows all the relevant information while still looking good on desktop and mobile.

Here is the mobile `/questions` page:  
![Questions page](./census-consensus-images/10-mobile.jpg)

Here is the bottom of the `/questions` page:  
![](./census-consensus-images/11.png)

It has the form for creating new questions.

Here is the rest of the form on the `/questions` page:  
![](./census-consensus-images/12.png)

Here is the `/questions/{id}/revisions` page:  
![](./census-consensus-images/13.png)

Includes the same info as the chapter revisions page, but for questions.

Here is the `/feedback` page, which was for writers and admins only:
![](./census-consensus-images/14.png)

As you can see, I used my private feedback to write my message for sending to potential collaborators. It also includes the logo clicks stats, more on this later.

Here is a screenshot of the only admin-only page, `/dashboard`:  
![](./census-consensus-images/15.png)

This has forms for adjusting the story ID, total population of Equestria, vote duration for all chapters, the event reset, and the start date and time.

Another fun fact about this story: The population of Equestria used for the event was taken from the start of a new save in [Hearts of Iron IV](https://store.steampowered.com/app/394360/Hearts_of_Iron_IV/) with the [Equestria at War](https://steamcommunity.com/sharedfiles/filedetails/?id=1826643372) mod.

All these screenshots were taken before the code was updating to make some of these pages public. A lot of site functionality will be removed by now to make most things read only.

## Late Development

Once writing had started, I worked on polishing the site and fixing bugs until it came time to code the event loop, the thing that controlled the event and updated the story with the correct chapter.

A fun fact about the website: The user [LastToTheParty](https://www.fimfiction.net/user/584567/LastToTheParty) is the last person to sign up as I write this. Their name checks out.

Now at this point in development we were getting really close to April 1st. The stress was getting to me and I had only written like nine questions for the event. I had to stop writing and go back to coding to get it all done in time.

Thankfully all the amazing people listed above helped out! [RunicTreetops](https://www.fimfiction.net/user/489485/RunicTreetops) wrote the three questions on The Friendship Chapter that I couldn't finish. [Math Spook](https://www.fimfiction.net/user/612387/Math+Spook) wrote the first and last chapters, while helping fix bugs in my code. [hawthornbunny](https://www.fimfiction.net/user/77473/hawthornbunny) helped connect chapters, add extra details, and find and fix formatting errors. [meadowsys](https://www.fimfiction.net/user/487213/meadowsys) coded the parser that converted our mess of a format into something readable that was posted on Fimfiction.

Literally the day of the event, 8 hours before launch or so, I added a page to preview chapters based off the current votes in the database. I also added a page to preview chapters with random votes. This was insanely useful for catching and fixing formatting errors. Absolutely crazy this wasn't implemented before the day of, considering how much it helped.

## Asset Showcase

Before we get to the live development during the event, let's go over the assets created for this project.

### Website Icons

[Math Spook](https://www.fimfiction.net/user/612387/Math+Spook) created the website icon based off an idea I had. They were some of the first things he made in [Inkscape](https://inkscape.org/). We dynamically serve the light or dark based off which theme you select or the browsers theme preference.

Celestia theme icon:  
![Celestia Favicon](https://census.silkrose.dev/assets/cc-light-512.png)

Luna theme icon:  
![Luna Favicon](https://census.silkrose.dev/assets/cc-dark-512.png)

### Story Covers

After Spook did such a good job making the icons, I decided to see what I could do in Inkscape to make a cover.

Celestia theme cover:  
![Celestia Cover](https://census.silkrose.dev/assets/cover-light.png)

Luna theme cover:  
![Luna Cover](https://census.silkrose.dev/assets/cover-dark.png)

The story on Fimfiction uses the Celestia themed cover, but I made a Luna version too.

Fun fact about the cover: I intentionally chose to showcase this question. I thought it would be funny to show a question that was never asked on any of the surveys. And Pinkie is in fact best pony.

## Live Event Development

After the event went live and seemed to be working, I breathed a sigh of relief. It didn't last long, though. After 2 surveys had completed, we noticed, and so did a reader in the comments, that the numbers weren't adding up.

We diagnosed the problem 15 minutes before the third survey went live. In my code for calculating the votes, I was putting them into buckets based on option IDs. I thought this function also sorted the votes beforehand, but I was wrong. This caused the bucket tallies to get overwritten every time the order of the votes and the order of the option IDs didn't match (which was almost all the time).

I added a sort into the database SQL statement and deployed the fix with only 6 minutes to spare. It worked and fixed the issue. We also used the website preview to get the correct chapter text for the already published chapters so that we could copy it over.

The second issue we ran into during the live event was on the 10th survey. The story's short description always showed one of the questions from the current survey. One particularly long question was so long that Fimfiction didn't allow it to be used as a short description. I was aware that this was a possible problem, but my string length limiting code had a bug. I was limiting the length, but I think it was one character too long. For 20 minutes my code was unable to update the story on Fimfiction, so I had to sit and manually operate the title countdown. After that question had passed, I fixed the code and deployed the fix.

A third issue we only discovered after the event: I forgot to save the API response to the database. I had wanted to showcase some graphs with metrics over time, but this data is lost to us.

## Chapters
Hello ponies! [hawthornbunny](https://www.fimfiction.net/user/77473/hawthornbunny) here. Silk Rose is busy coding (seriously, she's done way too much on this project and needs a break) so I'm taking over to talk about the chapters that we wrote for the event.

There was no overarching plan for the story - in fact the whole "Celestia gets the Mane Six to do a survey" framing wasn't even added until like the final day. There was a lot of confusion in the writing team about how the story was structured temporally - I thought we were writing the before and after threads separately, others thought the survey was somehow being taken and answered in real time, and it was all rather incoherent and a mess. So, while Silk Rose and Meadowsys were busy hammering at the code, us writers were busy hammering the story into shape, and luckily it all came together before the deadline.

Let's take a look at each chapter!

### [Inquiring Ponies Want to Know](https://www.fimfiction.net/story/589323/1/census-consensus/inquiring-ponies-want-to-know)
**Written by:** Math Spook

Math Spook basically carried us through the final day, supplying this first chapter that justifies the existence of the census and makes the whole thing work via vaguely-explained time magic. A handy excuse for our clumsy writing coordination. Spook talks about the chapter in his [behind the scenes blog](https://www.fimfiction.net/blog/1138770/behind-the-scenes-of-census-consensus) so you can read about it there, but for my part I thought the escalating cake binge was hilarious. FanOfMostEverything provided the chapter name.

### [The Friendship Chapter](https://www.fimfiction.net/story/589323/2/census-consensus/the-friendship-chapter)
**Written by:** Silk Rose, RunicTreetops, hawthornbunny

This was one of the first chapters written - a nice straightforward one about friendship to start us off. I thought it was a little plain, so I added a joke about Applejack mistaking "census" for "censors".

It's in this chapter that readers first get to see how their survey choices affect the writing. The system was designed so that no matter what readers chose, there would always be a response pre-written. Silk Rose's engine was designed to handle complex situations like "what if less than 30% chose option C", but I don't think any of us actually used anything more complex than "which option got the most votes". The responses all get compiled into one chapter that hopefully reads like a continuous narrative to the audience, which is why each chapter is just the characters reading through the answers one-by-one.

At the end of each chapter, there's an outro which is supposed to connect each chapter to the next, and we had to write them all pretty much on the last day because we didn't know what the chapter order was going to be. I had some fun with these as they were a chance to include a bit of actual continuity, like Rainbow Dash working on her own questions in the background.

Also I don't know if anyone noticed this:

> "Singing!" Pinkie nodded. "We do it all the time! I've always wanted to know how other ponies sing too, you know, when they're off - off somewhere else."

She was about to say "offscreen", but stopped just short of breaking the fourth wall.

### [Spontaneous Singing](https://www.fimfiction.net/story/589323/3/census-consensus/spontaneous-singing)
**Written by:** Written by Silk Rose, hawthornbunny.
**Proofread by:** Hipponous

Silk Rose's second chapter, but with significant additions by me. Silk's responses ended up coming out a bit short, so I took the opportunity to add an intro that shows how the time spell actually works. This also conveniently allowed me to double the amount of Derpy in the story, for no reason other than to make FanOfMostEverything happy.

There's a reference to *Inside Out*:

> "I knew it! How could it not be Joy? She's the best emotion!"
> 
> Applejack tilted her head, puzzled. "She?"

The outro for this chapter was written by Math Spook, as it leads into his economics chapter.

### [On apples, and the consumption thereof](https://www.fimfiction.net/story/589323/4/census-consensus/on-apples-and-the-consumption-thereof)
**Written by:** Math Spook

A chapter about Applejack and apples. I love how Applejack starts off with the pretense of being sensible but just goes completely off the rails when the subject is apples, and you can just see Twilight beginning to regret the entire endeavor. Again, see Spook's blog for more details, but I liked this line:

> Look at how many ponies want to buy apples! Why, 6209664 want to buy over six quintillion apples each! That’s more apples than have ever existed! Can you just imagine the sales?

### [The misery and sacrifice of fashionable mares](https://www.fimfiction.net/story/589323/5/census-consensus/the-misery-and-sacrifice-of-fashionable-mares)
**Written by:** Math Spook

And then Applejack has the gall to call Rarity's questions silly. Rarity's questions are more or less fine, it's just that she comes to the absolute stupidest conclusions from the data, like finding out which fashion opinions are the most popular so that they can deliberately ignore them next season. Also, Rarity is a walking chemical incident. I think this is where Spook started to hit his stride with the writing, as this one flows a lot more naturally even though it's composed from pre-written blocks.

I wrote the outro for this chapter, which has the characters all take a break for a bit. There were a few reasons for this: We'd already written later chapters that take place on different days, so I needed to establish that the time spell does actually allow for this. And secondly, I needed to separate Pinkie and Twilight, because Silk Rose's romance chapter has Twilight filling in her census answers as the outcomes are revealed, so she can't know about it beforehoof.

### [Romantic Research](https://www.fimfiction.net/story/589323/6/census-consensus/romantic-research)
**Written by:** Silk Rose, hawthornbunny
**Proofread by:** Hipponous, Math Spook

Silk Rose's romance chapter, AKA an excuse for Silk to get her OTP Twi-Pie into the story, a goal with which I am only too happy to assist. Pinkie's hastily-written questions, it turns out, are just her using the census to ask Twilight out, because if you've got access to government analytics and a time spell, why not make use of them?

After some smoochy time, Rainbow Dash's story arc comes to a point as she finally gets to ask the questions she's been working on. I took the liberty of having Rainbow also subvert the census system by submitting her questions without Twilight's oversight, because that was a convenient way to join it to the next chapter...

### [Sports in General](https://www.fimfiction.net/story/589323/7/census-consensus/sports-in-general)
**Written by:** RunicTreetops, hawthornbunny

...which opens with Twilight putting her hoof down and telling them not to submit any more census questions without her approval. She also slices an envelope open with a letter opener spell, a nod to the horrifying capabilities of unicorns.

Runic's prewritten responses did not include the actual questions being asked - a problem because without this context, the ponies would just seem to be reacting to nothing and it would be impossible to tell what's happening. I went in and added them myself, which is why the formatting of this chapter is different to the ones up until now.

Anyway, Dash's questions are about sports and they're all stupid, but in fairness, she's probably the *most likely* to write stupid questions, including the amazing **Why aren't you an athlete? (Yes/No)**.

At the end, Mayor Mare comes in, and - being a government official - she already knows about the census and has her own designs for it.

### [Political Polling](https://www.fimfiction.net/story/589323/8/census-consensus/political-polling)
**Written by:** Shay492

Shay gave us a break from the Mane Six asking the absolute dumbest questions possible, by having the canny Mayor Mare usurp the census for her own political ends. After all, if anypony knows how to wield an instrument of government, it's ol' Scrollflank. After a bit of taxpayer-funded, magic-accelerated polling, the Mayor leaves with her ill-gotten data, just in time for the true villain of the story to arrive...

... *Gen Z*.

Spook originally thought we'd reject the Skibidi Mark Crusaders for being too stupid, but we all loved it, and I particularly love the ominous way he introduced them. How many other stories will give you electoral analytics and skibidi in the same chapter? Don't answer that.

### [Skibidi Mark Crusaders](https://www.fimfiction.net/story/589323/9/census-consensus/skibidi-mark-crusaders)
**Written by:** Math Spook

I'll be honest, I still have no idea what the CMC were saying and it's funnier that way. It took me a few rereads to notice that Apple Bloom says nothing but "bruh" for almost the whole chapter. And introducing Luna of all ponies as the only one able to communicate on the CMC's level was pure genius. If you want to know what Luna was actually saying, check Spook's blog. soþlice.

I bet this line hit more readers than expected:

> Rainbow Dash said, “Face it, Twilight. From their perspective, we’re old folks now.”

I decided that the mind-breaking insanity of the Skibidi Mark Crusaders and Luna was the perfect excuse to have everypony take another break, which was good because I needed Twilight alone in her castle in order to lead into my chapter.

### [Elemental Efforts](https://www.fimfiction.net/story/589323/10/census-consensus/elemental-efforts)
**Written by:** hawthornbunny

This was the first chapter I wrote, and you can tell because the survey responses are kinda rubbish - there's only 4 questions, they all just rapid-fire one after the other and then they're done, without much in the way of humor. I got better at it later on, I think. But I do like the lead-in to this chapter, which had the Tree of Harmony commandeer Trixie's body so that it could talk to Twilight. This allowed me to include a little mythology gag, where Twilight tells the Tree to make its own avatar next time (not realizing that the Tree will end up just copying her form).

There's no real throughline from the census to the next chapter, because when I wrote my chapters I was under the impression that we were writing the before-census and after-census parts separately. So, it just cuts straight to Rarity in the spa, with Zephyr Breeze. Zephyr is here because a friend of mine adores the stallion, and thus I really wanted to include him. And the Zeph definitely wouldn't shy away from a spa visit, so we get him and Rarity shooting the breeze. Oops, bad phrasing.

Zeph tries to hit on Rarity because of course he does, but he also knows a lot about manes, which provides a perfect opportunity for them to collab on the next chapter and learn about the state of Equestrian haircare.

### [Quit Polling My Mane](https://www.fimfiction.net/story/589323/11/census-consensus/quit-polling-my-mane)
**Written by:** hawthornbunny

It's a pun, see. I really did write completely different responses for all the mane colors, but for whatever reason, everyone picked "None", so you got goths.

Of course, the *real* reason for this chapter is so that Zephyr can ask his completely stupid, self-serving question, and everyone voted for his ass because of course they did. Apparently I was the only one who didn't see that coming, and annoyingly that was the worst response I wrote. You shoulda voted for hooves and you'd have learned something about Applejack.

After that nonsense, FanOfMostEverything enters the battlefield! He wrote the outro, which is about Fluttershy shyly using the census to ask 60 million ponies about their pets.

### [Help Patrol the Pet Population](https://www.fimfiction.net/story/589323/12/census-consensus/help-patrol-the-pet-population)
**Written by:** FanOfMostEverything

I put this chapter after the Rarity/Zephyr one because the characters were already all off doing separate things and it made sense (you can argue that this is why Fluttershy isn't at the spa with Rarity and her brother). In this chapter, Twilight and Fluttershy enjoy some gentle friendly bonding as they go over the census results about pets, and Twilight tries to prevent Fluttershy going into a ponicidal rage. Even Fluttershy, it turns out, isn't immune to the pitfalls of poor survey design.

By a stroke of possibly fortune, Twilight and Fluttershy also happened to be the only characters present during Lyra's intro, so it made perfect sense for her chapter to come next. But how to join these two very tonally different scenes together? I needed to come up with something that could smoothly bridge the gap between this sweet scene of platonic love and Lyra's agitated raving. A segue so subtle, so silken, that readers wouldn't even perceive the transition.

Anyway, Lyra comes in ranting about humans, and she wants to use the census to find ponies she can trust. It's time for The H-Files.

### [The H-Files (insert spooky whistling theme music here)](https://www.fimfiction.net/story/589323/13/census-consensus/the-h-files-insert-spooky-whistling-theme-music-here)
**Written by:** Math Spook

Despite being a child of the 1990s, I never actually watched The X-Files. I was more into spaceships and lasers. Anyway, Starlight Glimmer makes a welcome return in this chapter, bringing some much-needed skepticism to counter Lyra's insane conspiratorial logic. This was my favorite bit:

> “Yes, many ponies who experience human abductions say they have a dreamlike quality,” said Lyra. “Nopony understands why.”
> 
> “Really?” asked Starlight Glimmer dryly. “Nopony has figured out what a dreamlike experience that happens while you sleep could possibly be?”

The story starts going completely off the rails at this point, assuming it was ever on them to begin with, as Maud unearths an ancient evil that turns Fluttershy into a giant mechanical monster, and Twilight decides this is a perfect topic for the next section of the census.

### [Terror of Mecha-Fluttershy](https://www.fimfiction.net/story/589323/14/census-consensus/terror-of-mecha-fluttershy)
**Written by:** Math Spook

I have seen a little bit of *Neon Genesis Evangelion*, enough to understand the whole joke with the traumatized child mecha pilot. This entire chapter is so much self-aware parody that it probably gained sentience, and I love it. My favorite part is the bit where Rainbow Dash gets traumatized slightly too late to be of any use.

We were near the end of the story at this point, and I'd decided to put my final chapter last, as it takes place in the far future. Since Spook's Mecha-Fluttershy chapter acts as a kind of soft ending to the nonsense in Ponyville, it felt like the right place to add this thousands-year time skip, which I'll discuss in the next section...

### [Generational Gap](https://www.fimfiction.net/story/589323/15/census-consensus/generational-gap)
**Written by:** hawthornbunny

I knew I wanted to get G5 into this story somehow, and I hit on the idea of Sunny and friends finding the census long after it had taken place and trying to make sense of it. This, in turn, led to the idea of the census being damaged and unreadable, and then I realized I could turn the joke around on the readers, by making them answer questions they couldn't actually read. There was one "correct" answer for each question, and all the other answers would obviously be stupid if you could actually read the question which you couldn't.

I wrote the lead-in to the chapter with deliberate mystery, not identifying the timeframe or any of the speakers until Pipp is mentioned by name. The whole "discovering the ruins of Ponyville" thing is inspired by [milesprower06](https://www.fimfiction.net/user/6403/milesprower06)'s [Forgotten](https://www.fimfiction.net/story/503677/forgotten) series, which I recommend.

Sadly, most people chose the correct answer ("chocolate") for the first question, which therefore got the most boring response, but the other three were suitably silly and reveal Sunny's crush on Princess Twilight which she totally doesn't have. Also, [Applejack is a silly pony](https://www.youtube.com/watch?v=84PZbL_-jjU).

The end of the chapter brings us - perhaps jarringly - back to the present, and the finale of the story. This was provided by Math Spook, who had the idea to end the census with a single, ominous request: assign a letter grade to the census. Dun dun dun.

### [Grading on a Curve](https://www.fimfiction.net/story/589323/16/census-consensus/grading-on-a-curve)
**Written by:** Math Spook

Princess Twilight may be a master of magic and a defeater of demons, but this chapter targets her single greatest weakness: *being graded*. I didn't actually know the term "grading on a curve" until I looked it up - apparently it refers to a system of relative grading where individuals are compared to others in the population. Hey, the "Math" in Spook's name isn't for show, you know.

Anyway, Twilight has been stuck in a loop of repeatedly fainting before she can hear the grade, which turns out to be almost a majority of As. Awwww, you guys. This was the last survey chapter, so there were no more questions to answer. All that's left is to go back to where we began with Celestia...

### [Census Consensus](https://www.fimfiction.net/story/589323/17/census-consensus/census-consensus)
**Written by:** Math Spook

And we end as we began - with a slice of cake, and Celestia praising a job (mostly) well done. See, there was a story arc there. Totally mapped and planned out like the Disney Star Wars trilogy. Pay no attention to that mare behind the curtain.

