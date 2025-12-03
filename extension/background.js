browser.menus.create({
    onclick: (info, tab) => {
        link = "mpv:"
        console.log(info, tab)
        if (info.linkUrl !== undefined) {
            link += info.linkUrl
        } else if (info.srcUrl !== undefined) {
            link += info.srcUrl
        } else {
            link += tab.url
        }
        browser.tabs.executeScript(
            tab.id,
            {code: `button = document.createElement("a")
            button.href = \"` + link + "\"\n" +
            `button.click()`
        })
    },
    title: "Open in MPV",
})
