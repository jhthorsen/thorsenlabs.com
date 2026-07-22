((thorsen) => {
  const phototStreamId = '{{ query.icloud_id | default(value = article.id) }}';
  const intervalId = setInterval(onScroll, 250);

  thorsen.openViewer = async (evt, inc) => {
    evt.preventDefault();
    pauseVideos();

    const {$viewer} = getContainers();
    if (inc === 'close') return $viewer.classList.remove('open');

    let [behavior, left] = ['instant', -1];
    if (inc === null) {
      const $figure = findFigure($viewer, evt.target.closest('[data-viewer-checksum]'));
      if (!$figure) return; // Clicked on something in .photostream-overview that does not have a dup
      left = $figure.offsetLeft;
      loadWebAsset($figure);
    } else {
      behavior = 'smooth';
      left = $viewer.scrollLeft + ($viewer.querySelector('figure').offsetWidth * inc);
    }

    if (left < 0) {
      $viewer.classList.remove('open');
    } else {
      $viewer.classList.add('open');
      $viewer.scrollTo({left, top: 0, behavior});
    }
  };

  async function fetchWebAssets(guids) {
    if (guids.length === 0) return [];

    const res = await fetch(`/photostream/${phototStreamId}/webassets`, {
      method: 'POST',
      headers: {'Content-Type': 'application/json;charset=UTF-8'},
      body: JSON.stringify(guids),
    });

    const json = await res.json();
    return json.items || [];
  }

  function generateViewer() {
    const {$overview, $viewer} = getContainers();
    if (!$overview || !$viewer) return;

    const observer = new IntersectionObserver((entries, _observer) => {
      if (!$viewer.classList.contains('open')) return;

      for (const entry of entries) {
        const $el = entry.target;
        if (entry.isIntersecting) {
          loadWebAsset($el);
          pauseVideos();

          // Scroll into view in overview page
          findFigure($overview, $el).scrollIntoView({behavior: 'instant', block: 'center'});
        }
      }
    }, {root: $viewer});

    for (const $figure of $overview.querySelectorAll('.image, .video')) {
      const $dup = document.createElement('figure');
      $dup.className = $figure.className + ' dvh dvw';
      $dup.dataset.viewerChecksum = $figure.dataset.viewerChecksum;
      $viewer.appendChild($dup);
      observer.observe($dup);
    }
  }

  function getContainers() {
    return {$overview: document.querySelector('.photostream-overview'), $viewer: document.querySelector('.photostream-viewer')};
  }

  function findFigure($container, $figure) {
    return $figure && $container.querySelector(`[data-viewer-checksum="${$figure.dataset.viewerChecksum}"]`);
  }

  function loadWebAsset($figure) {
    const src = $figure.dataset.src || '';
    const urlRe = /^[A-Za-z0-9-._~:/?#\[\]@!$&'()*+,;%=]+$/;
    if (!src.match(urlRe)) return; // Prevent XSS

    if ($figure.classList.contains('video')) {
      $figure.innerHTML = `<video controls playsinline src="${src}#t=0.001"></video>`;
    } else {
      $figure.innerHTML = `<img src="${src}" alt="">`;
    }

    delete $figure.dataset.src;
  }

  function pauseVideos() {
    for (const $video of document.querySelectorAll('video')) {
      $video.pause();
    }
  }

  async function onScroll() {
    const scrollY = Math.max(window.scrollY, 0);
    if (onScroll.y != undefined && Math.abs(onScroll.y - scrollY) < 20) return;
    if (onScroll.y == undefined) generateViewer();
    onScroll.y = scrollY;

    const {$overview, $viewer} = getContainers();
    if (!$overview || !$viewer) return clearTimeout(intervalId);

    let guids = [];
    for (const $figure of $overview.querySelectorAll('.image, .video')) {
      const isIntersecting
         = $figure.offsetTop >= scrollY - ($figure.offsetHeight * 4)
        && $figure.offsetTop <= scrollY + window.innerHeight + ($figure.offsetHeight * 4);

      if (isIntersecting) {
        $figure.classList.add('is-intersecting');
      } else {
        $figure.classList.remove('is-intersecting');
      }

      if ($figure.dataset.guid && guids.length < 20) {
        if (isIntersecting || guids.length) {
          guids.push($figure.dataset.guid);
          $figure.dataset.guid = '';
        }
      }
    }

    const items = await fetchWebAssets(guids);
    for (const checksum in items) {
      const $thumb = document.querySelector(`[data-thumb-checksum="${checksum}"]`);
      if (!$thumb) continue;

      const thumbItem = items[checksum];
      const $figure = findFigure($viewer, $thumb.closest('figure'));
      const viewerItem = items[$figure.dataset.viewerChecksum];
      $thumb.src = `https://${thumbItem.url_location}${thumbItem.url_path}`;
      $figure.dataset.src = `https://${viewerItem.url_location}${viewerItem.url_path}`;
    }
  }
})(window.thorsen || (window.thorsen = {}));
