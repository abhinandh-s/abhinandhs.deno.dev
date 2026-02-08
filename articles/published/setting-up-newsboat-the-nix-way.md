---
title: Nix Functions To Setup Newsboat Urls
published_at: 2024-10-22
updated_at: 2024-10-22
snippet: Setting up your newsboat configuration using Nix functions.
---

RSS is a powerful way to stay updated with your favorite blogs, social media
channels, and other online content without being distracted by endless scrolling
on the actual platforms. [Newsboat](https://newsboat.org/), a terminal-based RSS
reader, is an excellent tool for this.

In this post, I'll guide you through how to set up your newsboat configuration
using Nix functions to generate RSS feeds from platforms like YouTube, Twitter,
Reddit, Instagram, and Spotify with less boilerplates. For more informations on
supported platforms visit [rsshub](https://rsshub.netlify.app/).

# Setting Up Nix Functions for RSS Feeds

# Nix Functions

In Nix, functions are written as follows:

#### Single parameter

```json
param1: {
    # Function Body
}
```

#### Two parameters

```json
param1: param2: {
    # Function Body
}
```

Here, param1, param2, and so on are the inputs (arguments) to the function. This
allows you to write reusable functions where you only need to provide specific
data, such as a username or unique ID, and the function handles the boilerplate
parts of the links.

You might notice that functions in Nix don’t have a name. Nix functions are
similar to closures in Rust, This means you can’t call them directly by name,
they are anonymous. To use them, you must bind the function to a variable. Once
bound, you can call the variable like a named function.

Here’s an example:

### without funtions

```json
{ ... }:
let
in
{
    programs.newsboat = {
        enable = true;
        urls = [
            {tags = [ "articles" ]; url = "https://proton.me/blog/feed";}
            { # normal way 
                tags = [ "youtube" "music" ];
                url = "https://rsshub.app/youtube/user/@JFlaMusic";
            }
            { 
                tags = [ "youtube" "movies" ];
                url = "https://rsshub.app/youtube/user/@DanMurrellMovies";
            }
            { 
                tags = [ "youtube" "programming" ];
                url = "https://rsshub.app/youtube/user/@jonhoo";
            }
            { 
                tags = [ "youtube" "programming" ];
                url = "https://rsshub.app/youtube/user/@ThePrimeagen";
            }
            # and so on...
        ]
    };
}
```

### with funtions

```json
{ ... }:
let
    Youtube = userName: tags: {
        tags = tags;
        url = "https://rsshub.app/youtube/user/@${userName}";
    };
in
    {
    programs.newsboat = {
        enable = true;
        urls = [
            {tags = [ "articles" ]; url = "https://proton.me/blog/feed";}
            (Youtube "JFlaMusic" [ "youtube" "music" ])
            (Youtube "DanMurrellMovies" [ "youtube" "movies" ])
            (Youtube "jonhoo" [ "youtube" "programming" ])
            (Youtube "ThePrimeagen" [ "youtube" "programming" ])
            # and so on...
        ]
    };
}
```

This approach is much cleaner and simplifies debugging in the future. If the API
for RSS changes, you only need to update a single line in the function
definition, rather than rewriting all the occurrences.

# Final code

```json
{ ... }:
let
instagramFeeds = users: map ( userNames: {
    tags = [ "instagram" ];
    url = "https://rsshub.app/picuki/profile/${userNames}";
    }) users;
twitterFeeds = users: map ( userNames: {
    tags = [ "twitter" ];
    url = "https://nitter.privacydev.net/${userNames}/rss";
    }) users;
redditFeeds = users: map ( userNames: {
    tags = [ "reddit" ];
    url = "https://www.reddit.com/r/${userNames}.rss";
    }) users;
youTubeFeeds = users: map ( userNames: {
    tags = [ "youtube" ];
    url = "https://rsshub.app/youtube/user/@${userNames}";
    }) users;
Youtube = userName: tags: {
  tags = tags;
  url = "https://rsshub.app/youtube/user/@${userName}";
};
YoutubeId = channel_id: tags: {
  tags = tags;
  url = "https://www.youtube.com/feeds/videos.xml?channel_id=${channel_id}";
};
YtMusicChart = category: countryCode: tags: {
  tags = tags;
  url = "https://rsshub.app/youtube/charts/${category}/${countryCode}/RightNow";
};
Spotify = category: name: id: {
  tags = [ "music" ];
  url = "https://rsshub.app/spotify/${category}/${id}";
};
in 
{
  programs.newsboat = {
    enable = true;
    browser = "linkhandler";
    autoReload = false;
    urls = [
      {tags = [ "articles" ]; url = "https://feeds.feedburner.com/collabfund";}
      (Youtube "CinemaStellar" [ "youtube" "movies" ])
      (Youtube "DanMurrellMovies" [ "youtube" "movies" ])
      (YoutubeId "UC7YOGHUfC1Tb6E4pudI9STA" [])
      (YoutubeId "UCP7WmQ_U4GB3K51Od9QvM0w" [])
      (YtMusicChart "TopSongs" "in" [ "YtMusic" ])
      (Spotify "playlist" "Top Songs - Global" "37i9dQZEVXbNG2KDcFcKOF")
      (Spotify "playlist" "Top Songs - India" "37i9dQZEVXbMWDif5SCBJq")
      (Spotify "artist" "Halsey"            "26VFTg2z8YR0cCuwLzESi2")
      (Spotify "artist" "The Chainsmokers"  "69GGBxA162lTqCwzJG5jLp")
      ]
      ++ youTubeFeeds [
        "jonhoo"
        "LukeSmithxyz"
        "NoBoilerplate"
        ]
        ++ twitterFeeds [
          "jonhoo"
          "MikaPikaZo"
        ]
        ++ redditFeeds [
          "rust"
          "nixos"
          "unixporn"
        ]
        ++ instagramFeeds [
          "kartikaaryan"
        ];

    extraConfig = ''
      bind-key j down
      bind-key k up
      '';
  };
}
```
