import z from "zod";

export type Tag = {
	id: number;
	title: string;
	type: string;
	href: string;
	text: string;
};

export const id_schema = z.object({
	id: z.number(),
});

// Schema for the story API response.
export const api_schema = z.object({
	data: z.object({
		id: z.string(),
		type: z.string(),
		attributes: z.object({
			title: z.string(),
			short_description: z.string(),
			description: z.string(),
			description_html: z.string(),
			date_modified: z.string(),
			date_updated: z.string(),
			date_published: z.string(),
			published: z.boolean(),
			cover_image: z
				.object({
					thumbnail: z.string(),
					medium: z.string(),
					large: z.string(),
					full: z.string(),
				})
				.optional(),
			color: z.object({
				hex: z.string(),
				rgb: z.array(z.number()).min(3).max(3),
			}),
			num_views: z.number(),
			total_num_views: z.number(),
			num_words: z.number(),
			num_chapters: z.number(),
			num_comments: z.number(),
			rating: z.number(),
			status: z.string(),
			submitted: z.boolean(),
			completion_status: z.string(),
			content_rating: z.string(),
			num_likes: z.number(),
			num_dislikes: z.number(),
		}),
		relationships: z.object({
			author: z.object({
				data: z.object({
					type: z.string(),
					id: z.string(),
				}),
			}),
			tags: z.object({
				data: z.array(
					z.object({
						type: z.string(),
						id: z.string(),
					}),
				),
			}),
			prequel: z
				.object({
					data: z.object({
						type: z.string(),
						id: z.string(),
					}),
				})
				.optional(),
		}),
		links: z.object({
			self: z.string(),
		}),
		meta: z.object({
			url: z.string(),
		}),
	}),
	included: z.array(
		z.object({
			id: z.string(),
			type: z.string(),
			attributes: z.object({
				name: z.string(),
				bio: z.string(),
				bio_html: z.string(),
				num_followers: z.number(),
				num_stories: z.number(),
				num_blog_posts: z.number(),
				avatar: z.object({
					32: z.string(),
					48: z.string(),
					64: z.string(),
					96: z.string(),
					128: z.string(),
					160: z.string(),
					192: z.string(),
					256: z.string(),
					320: z.string(),
					384: z.string(),
					512: z.string(),
				}),
				color: z.object({
					hex: z.string(),
					rgb: z.array(z.number()).min(3).max(3),
				}),
				date_joined: z.string(),
			}),
			links: z.object({
				self: z.string(),
			}),
			meta: z.object({
				url: z.string(),
			}),
		}),
	),
	uri: z.string(),
	method: z.string(),
	debug: z.object({
		duration: z.string(),
	}),
});

// Schema for validating the JSON parsed from the HTML of the stats page.
export const stats_schema = z.object({
	chapters: z.array(
		z.object({
			date: z.string(),
			title: z.string(),
			views: z.string(),
			words: z.string(),
			words_text: z.string(),
			chapter_num: z.number(),
		}),
	),
	stats: z.object({
		data: z.array(
			z.object({
				views: z.number().optional(),
				likes: z.number().optional(),
				dislikes: z.number().optional(),
				date: z.string(),
			}),
		),
		first_chapter_date: z.string(),
		last_chapter_date: z.string(),
	}),
});
