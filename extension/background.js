browser.menus.create({
    onclick: (info, tab) => {
        link = ""
        if (info.linkUrl !== undefined) {
            link = info.linkUrl
        } else if (info.srcUrl !== undefined) {
            link = info.srcUrl
        } else {
            link = tab.url
        }
        browser.tabs.create({
            url: "mpv:" + link,
            active: false
        }).then((t) => {
            browser.tabs.remove(t.id)
        })
    },
    title: "Open in MPV",
})
