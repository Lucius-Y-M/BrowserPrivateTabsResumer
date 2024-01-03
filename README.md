#### Firefox Tab Resumer - A Terminal App (WIP)
## THIS IS STILL A WIP, THINGS MAY BREAK OR NOT WORK WITHOUT NOTICE

## What is this?

The intended purpose of this little terminal app is to help save links of tabs you've been visiting in a browser session (Firefox, Chrome etc.), _especially in Incognito / PrivateBrowsing Mode_, and then easily reopen them again in the browser of your choosing (__especially in Incognito / PrivateBrowsing Mode__), so you don't have to manually open a lot of tabs again.

It will provide at least the following functionality:

- Create, edit and delete "profiles" of links to be opened in tabs
- Manage a profile to open some or all tabs in normal or incognito mode
- Lookup links you've already added


I have done something similar in the past using Bash Script, but back then I only had a few tabs. While I'm sure the functionality listed above can be realized in Bash Script as well, it quickly becomes cumbersome once you have multiple "profiles", want to lookup a link, make sure you're not adding duplicate tabs, etc. Having a terminal app would be much more convenient. (Besides, any excuse to learn / write Rust is a good excuse, haha)


## But Firefox / Chrome already re-opens tabs from previous sessions by default?

Yes, **BUT** that only applies to Non-Incognito tabs.

## Wait wait wait, why would you want to keep tabs in Incognito mode?

I have always found the ubiquity of all those advertising & tracking cookies to be annoying (if not outright disgusting). Also, I don't want to search only once for something political on Youtube just to catch up with the news, and the next 20 times I visit Youtube the whole frontpage is full of CNN / FOX / alt-left / alt-right clickbaits.

The MOST use I get out of something like this, so far, is with Youtube: I can keep my main account free from all the **recommendation contamination** caused by searching for / watching content that you do not actually want to see on your homepage (If you want to DISABLE all Youtube homepage recommendations, that's another story; I can full-heartedly recommend [RYS - Remove Youtube Suggestions](https://addons.mozilla.org/en-CA/firefox/addon/remove-youtube-s-suggestions/)). But there are other benefits, such as the aforementioned avoidance of tracking cookies.
