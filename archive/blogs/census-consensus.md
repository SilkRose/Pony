# Census Consensus

Tagged story: [Census Consensus](https://www.fimfiction.net/story/589323/census-consensus)

***

With the release of this blog post, our [custom survey site](https://census.silkrose.dev/) has been updated to allow anyone to vote on all the surveys! It let's you view the current results for each chapter, and a random results page to see any combination of answers. If you missed the event, please go vote and make your voice heard!

In addition to allowing you to vote for missed surveys, you can also vote more than once to re-cast your ballot. We've also made the behind the scenes pages visible to the public. You can see exactly how chapters and questions were written and see every revision of them.

This story was a lot of fun, but it was also a lot of stress. The idea was chosen in early December, when I was naive and thought it'd only take a month to a month and a half to code everything. In mid December I made a group on Discord with myself and nine other people interested in the project. [Meadowsys](https://www.fimfiction.net/user/487213/meadowsys) started on the code at the end of December, and I joined her in coding in the middle of January. We ended up coding right up to and including April 1st. I even had to re-deploy twice to fix bugs while the event was happening. But, we'll talk about that later.

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
- [Silk Rose](https://www.fimfiction.net/user/237915/Silk+Rose)
- [Silver Needle](https://www.fimfiction.net/user/463467/Silver+Needle)

I guess I should explain how the event worked before going into more detail.

Once the story was live, a custom survey site allowed users to vote on a survey that would affect the outcome of a in-universe census that the mane 6 were holding. Everything from the questions, options, and results were all written before the event went live, so the code could publish the results for whatever options won.

If that didn't explain it well enough, you can read [Math Spook's](https://www.fimfiction.net/user/612387/Math+Spook) blog post: [Behind the scenes of “Census Consensus”](https://www.fimfiction.net/blog/1138770/behind-the-scenes-of-census-consensus), it does a great job of explaining the event. You should read it even if you understood my explanation, it's a good read and he's a great writer.

Let's start at the beginning, late last year [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus) told me he wasn't planning to do an April Fools event this year. I'd helped him do two of these in the past, so I felt like I could step up and do something instead. I mean how hard could it be? The last two years, while hectic, weren't *that* bad…

Now, while in the middle of explaining this story, let's go back even farther to the first April Fools event I helped with: [This Story Did Not Explode](https://www.fimfiction.net/story/553695/this-story-did-not-explode), or as we called it internally: The Exploding Story. I was the one who came up with the idea. We both immediately liked the idea and went forward with it. [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus) wrote it while I coded it with help from [meadowsys](https://www.fimfiction.net/user/487213/meadowsys) for the timing code. I also wrote two chapters. Had to insert romance into it some how, right?

This story exploded onto the scene, immediately getting attention. The event was awesome, and ran without a hitch. Well, at least my code went off without a hitch, you can read more about this event in Bob's blog on it: [The Real Explosion Was The Friends We Made Along The Way](https://www.fimfiction.net/blog/1036675/the-real-explosion-was-the-friends-we-made-along-the-way). I also wrote a blog post about the code, you can read it here: [The Exploding Story Code Overview](https://www.fimfiction.net/blog/1036674/the-exploding-story-code-overview).

A fun fact about this story: while it was live, an archival friend of mine messaged me to ask if was really going to explode, because he wanted to know if he had to stay up all night to archive the story so it wouldn't be lost.

I bring up The Exploding Story to say: this is where it all began. I won't speak for Bob, but for me this is where I realized a story could be more than just a story, it could be an event. We immediately knew that whatever we did next year, we wanted it to be interactive. This was something we'd always wanted, but didn't have the time for this first event.

Next year's event, was exactly that, interactive. [Democracy Manifest](https://www.fimfiction.net/story/575601/democracy-manifest), known as The Democracy Story, was interactive, it used the like button as a way to vote on proposals in the story. I wrote the code while Bob did the writing, but this time with some outside help. I also wrote two chapters.

Something interesting about the code that only I utilized was: it allowed branching paths. I wrote the chapter where you vote on if Pinkie or Fluttershy are cuter. Then whoever is voted less cute, goes back to the voting ponies to ask them if she should ask out the other pony. If you'd like to read the alternative chapters, you can do so [here](https://github.com/SilkRose/Pony/blob/mane/stories/democracy-manifest/democracy-manifest-meta.md) in my [Pony](https://github.com/SilkRose/Pony) repository.

A fun fact about this story: no matter who you vote as cutest, or if you vote for them to ask out the other, I made it so they always ended up together.

I recently re-read this story, and I really liked this line I wrote in it:
> Fluttershy smiles, appearing more relaxed after getting that off her chest. "She's just so amazing, I really want to ask her out, but I don't know if I could do it without the support of a bunch of ponies I don't know who agree that she is cuter than me."

You might have noticed that neither Bob or I wrote a blog about this story. Bob was burnt out from writing, and was a little disappointed with it, so he didn't want to write a blog. I didn't want to write one unless Bob was going to, and the code was messier and would be harder to explain.

This story had what can only be described as an obvious flaw in its design: you can only like a story once. With the fact removing a like counted as a no vote, this made interacting with the mechanics of the story very clunky. There was a five minute window between chapters you could remove a like to get your vote back, but I don't know how many people realized this.

If I had to rank these two stories in terns of fun and enjoyment, I'd put The Exploding Story above The Democracy Story. While they were both fun, The Democracy Story didn't have that explosive energy of the first one.

Now, let's get back to this year's event: [Census Consensus](https://www.fimfiction.net/story/589323/census-consensus).

For the last 2 years, I had someone to call the shots for me. Bob was an amazing coordinator for these April Fools events. This was the first event I had to lead the ship myself.

I wanted this whole project to be as collaborative as I could get from the very start. I didn't want to arbitrarily decide anything unless it was absolutely necessary. This is why the Discord group was made before any code had even been written.

The first thing I coded for the project was the database SQL statements, over a thousand lines of create, delete, update, insert SQL. Once I got the table creation statements done, [PseudoBob Delightus](https://www.fimfiction.net/user/12771/PseudoBob+Delightus) helped me with fixing the table designs. He says he didn't help with this event, but he did help with this in the very beginning.

The original plan for the code was for myself to code the back end server stuff, while [meadowsys](https://www.fimfiction.net/user/487213/meadowsys) coded the front end using a specific library called [Leptos](https://www.leptos.dev/). Unfortunately, we never got to the point of converting my back end HTML template code to the new system, as we ran out of time. And the code I had written on the back end using an HTML template library called [Maud](https://maud.lambda.xyz/) was functioning well. Yes, that library's name is a MLP reference.

A fun fact about this story: [hawthornbunny](https://www.fimfiction.net/user/77473/hawthornbunny) thought of a question idea early on about a pony working in the factory the census was being printed in getting stuck and asking a question on the survey as a way to call for help. We never found a way to use this, but we all liked the idea.

By the time the database code was done, meadow had authentication working and I could start working on the pages for writers to create and edit questions and chapters.