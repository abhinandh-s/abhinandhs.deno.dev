---
title: How to Set Up GoDaddy Domain With GitHub Pages
published_at: 2024-10-19
updated_at: 2024-10-19
snippet: How to Set Up GoDaddy Domain With GitHub Pages.
---

Congrats! You’ve finally bought yourself a domain, and now you're ready to
connect it to your GitHub Pages website. This guide will walk you through the
steps to set up your GoDaddy domain with GitHub Pages.

# Step 1: Configuring GitHub Pages

First, ensure that your repository name follows the required naming convention
for GitHub Pages ie, your repository name must be your-username.github.io.

mine is: https://github.com/abhi-xyz/abhi-xyz.github.io

Once your repository is ready, navigate to the repository’s Settings.

## Enable GitHub Pages

1. Go to the Settings of your repository.
2. Scroll down to the Code and Automation section and select Pages.
3. Under Source, choose the branch that contains your static site. For most
   cases, this will be the main or master branch.
4. For the directory, choose root (ie, the directory which the index.html is
   situated).
5. Save the changes, and GitHub will start serving your site from
   your-username.github.io.

# Step 2: Setting Up DNS Records in GoDaddy

Now that your GitHub Pages site is live, the next step is to point your custom
GoDaddy domain to GitHub Pages.

## Add DNS Records in GoDaddy

1. Log in to your [GoDaddy account](https://www.godaddy.com/).
2. Under **My Products**, find your domain and click DNS to manage its DNS
   settings.
3. Add the following entries in the DNS management settings:

![add these entries in GoDaddy's DNS management settings](/static/articles/how-to-set-up-GoDaddy-domain-with-gitHub-pages/01-godaddy-dns-management.avif)

- Type: A
- Name: @
- Value: 185.199.108.153 (Values might change so refer this
  [instructions](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site/managing-a-custom-domain-for-your-github-pages-site#configuring-an-apex-domain)
  and put aprropriate values.)

(Repeat this for all four IPs: 185.199.108.153, 185.199.109.153,
185.199.110.153, 185.199.111.153)

- TTL: 600 seconds (default is fine)

4. Add a CNAME record as well: (optional if you want to set www with
   yourdomain.com. ie, www.yourdomain.com)

- Type: CNAME
- Name: www
- Value: your-username.github.io
- TTL: 600 seconds

Save the changes.

# Step 3: Configuring a Custom Domain in GitHub Pages

Once you’ve set up the DNS records in GoDaddy, go back to your GitHub
repository.

Head to Settings > Pages.

![github-pages](/static/articles/how-to-set-up-GoDaddy-domain-with-gitHub-pages/01-github-pages.avif)

Scroll down to the Custom Domain section.

![github-pages settings](/static/articles/how-to-set-up-GoDaddy-domain-with-gitHub-pages/01-github-pages-settings.avif)

Enter your GoDaddy domain (e.g., yourdomain.com) in the field. GitHub will
automatically create a CNAME file in the root directory of your repository. This
file tells GitHub that your website is linked to this custom domain.

![generated CNAME file in root](/static/articles/how-to-set-up-GoDaddy-domain-with-gitHub-pages/01-docs-folder.avif)

After adding the domain, GitHub Pages will take a few minutes to process the
changes.

# Step 4: Check SSL/TLS and HTTPS

After configuring your custom domain, it's crucial to ensure that your website
is served securely over HTTPS. GitHub Pages offers built-in support for HTTPS
through Let's Encrypt, which is a major advantage over traditional domain
providers like GoDaddy.

1. Go to your repository's Settings.
2. In the GitHub Pages section, look for the option to Enforce HTTPS.
3. Enable the Enforce HTTPS checkbox. This will ensure that visitors are
   automatically redirected to the secure version (HTTPS) of your site.

However, after adding your domain, you may encounter a message saying, "TLS
certificate is being provisioned". This is completely normal and indicates that
GitHub is working with Let's Encrypt to issue your SSL certificate. You may need
to refresh the page after some time if the Enforce HTTPS option is disabled at
first.

![TLS certificate provisioned](/static/articles/how-to-set-up-GoDaddy-domain-with-gitHub-pages/tls_provisioned.avif)

## Why HTTPS Matters

Normally, domains purchased from GoDaddy or other providers are routed through
HTTP by default, which means your site won't be secure. When someone visits your
site, their browser will display a warning indicating that the site is not
secure, which can drive away visitors.

If you were to purchase a SSL certificate from GoDaddy, it could cost you
approximately ₹4999 annually. However, by using GitHub Pages, you get a free TLS
certificate through Let's Encrypt, giving you secure, encrypted access to your
website without additional costs. This is one of the key benefits of using
GitHub Pages for hosting!

Once the DNS settings propagate (which may take between 24-48 hours), your
GitHub Pages site will be securely accessible through your custom GoDaddy domain
over HTTPS.

That’s it! You've successfully set up your GoDaddy domain with GitHub Pages. 🎉
Enjoy your new, secure website!
