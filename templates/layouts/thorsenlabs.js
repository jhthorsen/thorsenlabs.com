function notification(message) {
  return `<div class="notification" data-init="@debounce(@destroy(el), 3000)" data-swap="append:#notifications">${message}</div>`
}

((thorsen) => {
  let header = null, headerHeight = 0, lastScroll = 0

  window.addEventListener('scroll', () => {
    const currentScroll = window.pageYOffset

    if (header === null || header.parentNode === null) {
      header = document.querySelector('#header')
      headerHeight = header.offsetHeight
    }

    if (!currentScroll) {
      header.classList.remove('animate')
      header.classList.remove('scrolled')
      header.classList.remove('show')
    }
    else if (Math.abs(currentScroll - lastScroll) < headerHeight) {
      return
    }
    else if (currentScroll < lastScroll) {
      header.classList.add('animate')
      header.classList.add('show')
    }
    else {
      header.classList.add('scrolled')
      header.classList.remove('show')
    }

    lastScroll = currentScroll
  })

  thorsen.syntaxHighlight = async () => {
    const $code_blocks = document.querySelectorAll("pre > code")
    if ($code_blocks.length === 0) return

    const {codeToHtml} = await import('https://esm.sh/shiki@1.0.0')
    for (const $code_block of $code_blocks) {
      try {
        const lang = $code_block.className.replace(/.*language-(\w+).*/, "$1")
        const $pre = $code_block.closest("pre") || $code_block
        $pre.outerHTML = await codeToHtml($code_block.innerText, {
          lang: lang || "yaml",
          theme: "tokyo-night",
        })
      } catch (err) {
        console.error($code_block, err)
      }
    }
  }

  document.addEventListener('ssr:init', () => thorsen.syntaxHighlight())
  thorsen.syntaxHighlight()
})(window.thorsen || (window.thorsen = {}));
