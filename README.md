# intercollegiatemathtournament.org

This is the repository for 
[ICMT](intercollegiatemathtournament.org).

One of the primary goals is to make it easy for anyone to add and update
content. To that end, the site is built with [Zola](https://getzola.org/), a
static-site-generator that renders content from Markdown files.

There is currently little support for indexed markdown files, but I expect this will change soon. Currently most of the website is written in straight HTML.

## Adding content

TBD. 


## Writing Markdown

These content files are written in Markdown, which is a lightweight way to add
formatting to plain text documents. If you've sent bold text on Discord (or many
other platforms), you've already used Markdown!

For those unfamiliar, a cheat sheet is available
[here](https://commonmark.org/help/).

In addition to standard Markdown, we provide the following special components,
called _shortcodes_:

<table>
  <thead>
    <th>Shortcode</th>
    <th>Example</th>
    <th>Rendered</th>
  </thead>
  <tbody>
    <td>Button</td>
    <td>
<pre>
{% button_link(href="https://contestdojo.com/") %}
Register on ContestDojo
{% end %}
</pre>
    </td>
    <td>
      <img src="button-example.png">
    </td>
  </tbody>
</table>

Every Markdown file in this repository also has a special block at the very top
called _front matter_, delimited by `+++`. Here, you can specify additional
properties for each page, such as the title or date.

If you're adding a new page and not sure what to put for front matter, try
looking at a page similar to the one you're adding and basing it on that. Feel
free to ask the Tech team any questions.

## Editing templates

Changing how content is rendered or adding special elements (like the banner on
the home page) may require editing a template. Templates are written in
[Tera](https://keats.github.io/tera/), a template engine for HTML.

The default template is `page.html` for pages and `section.html` for section
listings (like the News tab which lists all the posts). The home page has its
own `index.html` template.

Most pages are based on the `base.html` template. Common changes, such as adding
a tab to the tab bar, probably belong here.

## Running the site locally

It is recommended to test the site using the [Nix package manager]:

```bash
nix develop -c just build
nix develop -c just serve
```

If you don't have Flakes configured globally, you may need to pass in
`--extra-experimental-features "nix-command flakes"`.

A development server should now be running at http://localhost:1111. It should
update automatically as you edit the site, so this is suitable for development.


## Deploying

Deploying is done through github actions so deploying manually shouldn't be needed. That being said, here are the steps:

It is recommended to build the site using the [Nix package manager]:

For those looking to install Nix, it is recommended to use the [Deterministic Systems Installer](https://determinate.systems/nix-installer/) in the case that you decide to uninstall Nix in the future.

```bash
nix build
```

This will make a static build of the site and place the files in the `public/`
folder. Simply copy these files to a web host using your tool of choice to
deploy.

To make a production build for use at https://berkeley.mt/, run:

```bash
nix build .#for-production
```

This sets the `base_url` option correctly, so that SEO is optimized.

Commits to the `main` branch on the main repository will automatically be
deployed to the live site. Look out for little green check marks next to commits
on GitHub!

[Nix package manager]: https://nixos.org/
[Zola]: https://getzola.org/
[Tailwind CSS]: https://tailwindcss.com/blog/standalone-cli