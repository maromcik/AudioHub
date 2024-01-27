let playerIntervalId = 0;
let currentBookId = -1;

const getCurrentPlayerTime = () => {
    return document.getElementById('audiobook-player').currentTime;
}

const getBeginningPlayerTime = () => {
    return document.getElementById('audiobook-player').getAttribute("begin-time");

}
// player init
document.addEventListener("htmx:load", (e) => {
    if (e.detail.elt.id === 'player-container') {
        clearInterval(playerIntervalId);
        let audio = document.getElementById('audiobook-player');
        let bookId = audio.lastElementChild.id;
        attachInterval(parseBookIdFromSource(bookId));

        audio.play();

        let beginTime = getBeginningPlayerTime();
        // initially set active book to current selection
        fetch(`/audiobook/${currentBookId}/active?position=${beginTime}`, {
            method: "PUT",
        });
        audio.currentTime = beginTime;

        audio.onpause = () => {
            clearInterval(playerIntervalId);
        }

        audio.onended = () => {
            clearInterval(playerIntervalId);
            updateActiveBook()
        }
    }
});


const attachInterval = (bookId) => {
    currentBookId = bookId;
    document.getElementById('audiobook-player').onplay = () => {
        createInterval(bookId)
    }
};

// TODO: prolong interval
const createInterval = () => {
    playerIntervalId  = setInterval(() => {
        updateActiveBook()
    }, 3_000);
}

const parseBookIdFromSource = (sourceId ) => {
    return sourceId.slice(7, sourceId.length);
};

const updateActiveBook = () => {
    fetch(`/audiobook/${currentBookId}/active?position=${getCurrentPlayerTime()}`, {
        method: "PUT",
    });
}

const attachHideQuickSearchListener = () => {
    document.body.addEventListener('click', hideQuickSearchResults)
}
const hideQuickSearchResults = () => {
    document.getElementById('quick-search-input').value = '';
    document.getElementById('search-result').style.display = 'none';
    document.body.removeEventListener('click', this);
}


document.addEventListener("htmx:afterRequest", (e) => {
    // rating clear
    if (e.detail.elt.id === 'rating-form') {
        document.getElementById('rating-form').reset();
    }

    // update rating summary
    if (e.detail.target.id === 'my-rating-container' && e.detail.elt.id === 'rating-form') {
        htmx.trigger('#ratings-summary-container', 'change')
    }

    // after calling create chapter
    if (e.detail.elt.id === 'create-chapter-form') {
        document.getElementById('create-chapter-form').reset();
        htmx.trigger("#chapters-list", "studio-form-submit");
        htmx.trigger("#chapters-timeline", "studio-form-submit");
    }

    // after calling delete chapter
    if (e.detail.target.id === 'chapters-list' && e.detail.elt.id === 'chapters-container') {
        htmx.trigger("#chapters-timeline", "studio-form-submit");
    }
});

const increasePage = (maxPage, minPage) => {
    htmx.trigger("#load-next-page-btn", "page-change");
    const currentPage = document.getElementById('pagination-text').getAttribute('current-page');
    const nextPage = Math.min(parseInt(currentPage) + 1, maxPage);
    setCurrentPageDialog(nextPage, maxPage);
    setNextButtonLink(nextPage, minPage, maxPage);
    setPrevButtonLink(nextPage, maxPage);
}

const decreasePage = (maxPage, minPage) => {
    htmx.trigger("#load-previous-page-btn", "page-change");
    const currentPage = document.getElementById('pagination-text').getAttribute('current-page');
    const nextPage = Math.max(parseInt(currentPage) - 1, 1);
    setCurrentPageDialog(nextPage, maxPage);
    setNextButtonLink(nextPage, minPage, maxPage);
    setPrevButtonLink(nextPage, maxPage);
}

const setCurrentPageDialog = (currentPage, maxPage) => {
    const pagination = document.getElementById('pagination-text');
    pagination.setAttribute('current-page', currentPage);
    pagination.innerText = `Page ${currentPage} out of ${maxPage}`;
}

const setPrevButtonLink = (pageNum, maxPage) => {
    const button = document.getElementById("load-previous-page-btn");
    const hxUrl = button.getAttribute('hx-get');
    const parts = hxUrl.split('=');
    let nextPage = pageNum - 1;
    nextPage = Math.min(nextPage, maxPage)
    nextPage = Math.max(1, nextPage)
    button.setAttribute('hx-get', `${parts[0]}=${nextPage}`)
    htmx.process(button)
}

const setNextButtonLink = (pageNum, minPage, maxPage) => {
    const button = document.getElementById("load-next-page-btn");
    const hxUrl = button.getAttribute('hx-get');
    const parts = hxUrl.split('=');
    let nextPage = pageNum + 1;
    nextPage = Math.min(nextPage, maxPage)
    nextPage = Math.max(minPage, nextPage)
    button.setAttribute('hx-get', `${parts[0]}=${nextPage}`)
    htmx.process(button)
}