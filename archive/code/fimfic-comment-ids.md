# FIMFiction Comment IDs

This guide walks you through the steps to collect IDs for comments on a story on FIMFiction for replying to them all at once.

This works because changing the page of comments you're on doesn't reset the console.

You need to open the console on your browser, and follow along with the steps below.

To start, we initialize a variable for comments, set the story URL we want to scrape, and get the total page count for comments.

```javascript
let comments = [];
let story_url =
	// This is an example URL:
	"https://www.fimfiction.net/story/553695/this-story-did-not-explode";
let pages = parseInt(
	document.querySelector("ul[data-num_pages]").dataset.num_pages
);
```

We are going to collect the comments with the following function.

```javascript
function get_comment_data() {
	document.querySelectorAll(".comment").forEach((comment) => {
		if (comment.firstElementChild.classList.contains("message")) return;
		const author_link = comment.querySelector(".author a.name");
		const author_id = author_link.getAttribute("href").split("/")[2];
		const comment_link = comment.querySelector(
			".data .comment_information .meta a:not(.name)"
		);
		const comment_id = comment_link.getAttribute("href").split("/").pop();
		comments.push({ author_id, comment_id });
	});
}
```

Now, you just need to go to every page and run that function, but we will be using another function for this.

```javascript
async function load_pages(pages) {
	for (let i = 1; i <= pages; i++) {
		window.location.href = `${story_url}#page/${i}`;
		await new Promise((resolve) => setTimeout(resolve, 1000));
		get_comment_data();
	}
}
```

After that, we have to await that function.

```javascript
await load_pages(pages);
```

Once you've done that, we will remove comments so that only one comment per user is left in the list.

```javascript
let author_ids = [];
let comment_ids = [];
comments.forEach((c) => {
	if (author_ids.includes(c.author_id)) return;
	author_ids.push(c.author_id);
	comment_ids.push(c.comment_id);
});
```

Now we simply format the IDs for replying.

```javascript
let comment_string = ">>" + comment_ids.join(" >>");
```

To easily copy the string to our clipboard, we have to add an event listener.

```javascript
document.addEventListener("click", () => {
	navigator.clipboard.writeText(comment_string);
});
```

That's it, all you have to do is click anywhere on the page, and you now have your giant list of IDs formatted for mass replying on FIMFiction.
