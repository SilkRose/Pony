export const story_index_table = `CREATE TABLE IF NOT EXISTS Story_index (
	story_id            integer     PRIMARY KEY,
	status              text        NOT NULL,
	version             integer     NOT NULL,
	timestamp           integer     NOT NULL
)`;

export function insert_story_index(
	id: number,
	status: string,
	version: number,
	timestamp: number,
) {
	return `INSERT OR IGNORE INTO Story_index (story_id, status, version, timestamp) 
	VALUES (${id}, '${status}', ${version}, ${timestamp})`;
}

export const authors_table = `CREATE TABLE IF NOT EXISTS Authors (
	id                  integer     PRIMARY KEY,
	name                text        NOT NULL,
	date_joined         integer     NOT NULL,
	followers           integer     NOT NULL,
	blogs               integer     NOT NULL
)`;

export function insert_author(
	id: number,
	name: string,
	date_joined: number,
	followers: number,
	blogs: number,
) {
	return `INSERT OR IGNORE INTO Authors (id, name, date_joined, followers, blogs) 
	VALUES (${id}, '${name}', ${date_joined}, ${followers}, ${blogs})`;
}

export const stories_table = `CREATE TABLE IF NOT EXISTS Stories (
	id                  integer     PRIMARY KEY,
	title               text        NOT NULL,
	date_modified       integer     NOT NULL,
	date_updated        integer     NOT NULL,
	date_published      integer     NOT NULL,
	cover               integer     NOT NULL,
	color_hex           integer     NOT NULL,
	views               integer     NOT NULL,
	total_views         integer     NOT NULL,
	num_comments        integer     NOT NULL,
	rating              integer     NOT NULL,
	completion_status   text        NOT NULL,
	content_rating      text        NOT NULL,
	likes               integer     NOT NULL,
	dislikes            integer     NOT NULL,
	ranking             integer     NOT NULL,
	word_ranking        integer     NOT NULL,
	bookshelves         integer     NOT NULL,
	tracking            integer     NOT NULL,
	author_id           integer     NOT NULL,
	prequel_id          integer,

	CONSTRAINT stories_author_id_fk FOREIGN KEY (author_id)
   	REFERENCES Authors (id),
	
	CONSTRAINT story_index_id_fk FOREIGN KEY (id)
   	REFERENCES Story_index (story_id)
)`;

export function insert_story(
	id: number,
	title: string,
	date_modified: number,
	date_updated: number,
	date_published: number,
	cover: number,
	color_hex: string,
	views: number,
	total_views: number,
	num_comments: number,
	rating: number,
	completion_status: string,
	content_rating: string,
	likes: number,
	dislikes: number,
	ranking: number,
	word_ranking: number,
	bookshelves: number,
	tracking: number,
	author_id: number,
	prequel_id: number | "NULL",
) {
	return `INSERT OR IGNORE INTO Stories (
		id, title, date_modified, date_updated, date_published,
		cover, color_hex, views, total_views, num_comments,
		rating, completion_status, content_rating,
		likes, dislikes, ranking, word_ranking,
		bookshelves, tracking, author_id, prequel_id)
	VALUES (
		${id}, '${title}', ${date_modified}, ${date_updated}, ${date_published},
		${cover}, '${color_hex}', ${views}, ${total_views}, ${num_comments},
		${rating}, '${completion_status}', '${content_rating}',
		${likes}, ${dislikes}, ${ranking}, ${word_ranking},
		${bookshelves}, ${tracking}, ${author_id}, ${prequel_id})`;
}

export const tags_table = `CREATE TABLE IF NOT EXISTS Tags (
	id                  integer     PRIMARY KEY,
	title               text        NOT NULL,
	type                text        NOT NULL,
	text                text        NOT NULL,
	href                text        NOT NULL
)`;

export function insert_tag(
	id: number,
	title: string,
	type: string,
	text: string,
	href: string,
) {
	return `INSERT OR IGNORE INTO Tags (id, title, type, text, href) 
	VALUES (${id}, '${title}', '${type}', '${text}', '${href}')`;
}

export const tag_links_table = `CREATE TABLE IF NOT EXISTS Tag_links (
	story_id            integer,
	tag_id              integer,

	CONSTRAINT tag_links_story_id_fk FOREIGN KEY (story_id)
		REFERENCES Stories (id),

	CONSTRAINT tag_links_tag_id_fk FOREIGN KEY (tag_id)
		REFERENCES Tags (id),

	CONSTRAINT tag_links_pk PRIMARY KEY (story_id, tag_id)
)`;

export function insert_tag_link(story_id: number, tag_id: number) {
	return `INSERT OR IGNORE INTO Tag_links (story_id, tag_id) 
	VALUES (${story_id}, ${tag_id})`;
}

export const chapters_table = `CREATE TABLE IF NOT EXISTS Chapters (
	story_id            integer     NOT NULL,
	chapter_num         integer     NOT NULL,
	title               text        NOT NULL,
	date_modified       integer     NOT NULL,
	views               integer     NOT NULL,
	words               integer     NOT NULL,

	CONSTRAINT chapter_story_id_fk FOREIGN KEY (story_id)
   	REFERENCES Stories (id),
	
	CONSTRAINT chapters_pk PRIMARY KEY (story_id, chapter_num)
)`;

export function insert_chapter(
	story_id: number,
	chapter_num: number,
	title: string,
	date_modified: number,
	views: number,
	words: number,
) {
	return `INSERT OR IGNORE INTO Chapters (
		story_id, chapter_num, title,
		date_modified, views, words) 
	VALUES (
		${story_id}, ${chapter_num}, '${title}',
		${date_modified}, ${views}, ${words})`;
}

export const stats_table = `CREATE TABLE IF NOT EXISTS Stats (
	story_id            integer     NOT NULL,
	date                integer     NOT NULL,
	views               integer,
	likes               integer,
	dislikes            integer,

	CONSTRAINT stats_story_id_fk FOREIGN KEY (story_id)
   	REFERENCES Stories (id),
	
	CONSTRAINT stats_pk PRIMARY KEY (story_id, date)
)`;

export function insert_stats(
	story_id: number,
	date: number,
	views: number | "NULL",
	likes: number | "NULL",
	dislikes: number | "NULL",
) {
	return `INSERT OR IGNORE INTO Stats (
		story_id, date, views, likes, dislikes) 
	VALUES (
		${story_id}, ${date}, ${views}, ${likes}, ${dislikes})`;
}

export const referral_sites_table = `CREATE TABLE IF NOT EXISTS Referral_sites (
	id                  integer     PRIMARY KEY,
	site                string      NOT NULL,

	UNIQUE(site)
)`;

export function insert_referral_site(site: string) {
	return `INSERT OR IGNORE INTO Referral_sites (site) 
	VALUES ('${site}')`;
}

export function retrieve_referral_site_id(site: string) {
	return `SELECT id FROM Referral_sites WHERE site = '${site}';`;
}

export const referrals_table = `CREATE TABLE IF NOT EXISTS Referrals (
	story_id            integer     NOT NULL,
	referral_site_id    integer     NOT NULL,
	count               integer     NOT NULL,

	CONSTRAINT referrals_story_id_fk FOREIGN KEY (story_id)
   	REFERENCES Stories (id),
	
	CONSTRAINT referrals_referral_site_id_fk FOREIGN KEY (referral_site_id)
   	REFERENCES Referral_sites (id),
	
	CONSTRAINT referrals_pk PRIMARY KEY (story_id, referral_site_id)
)`;

export function insert_referral(
	story_id: number,
	referral_site_id: number,
	count: number,
) {
	return `INSERT OR IGNORE INTO Referrals (
		story_id, referral_site_id, count) 
	VALUES (
		${story_id}, ${referral_site_id}, ${count})`;
}
