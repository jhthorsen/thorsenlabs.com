((thorsen) => {
  const PAC_FUNCTIONS = {
    alert: true,
    dateRange: false,
    dnsDomainIs: true,
    dnsDomainLevels: true,
    dnsResolve: true,
    isInNet: true,
    isPlainHostName: true,
    isResolvable: true,
    localHostOrDomainIs: true,
    myIpAddress: true,
    shExpMatch: true,
    timeRange: false,
    weekdayRange: false,
  };

  class ProxyForURL {
    attach(d) {
      this.form = d.querySelector('#proxy_for_url');
      if (!this.form) return console.warn('Could not find form#proxy_for_url');

      this.logEl = d.querySelector('table#pac_log, form table');
      this.myIpAddressInput = this.form.querySelector('[name=my_ip_address]');
      this.rulesInput = this.form.querySelector('[name=rules]');
      if (!this.myIpAddressInput || !this.rulesInput) return console.error('Could not find all form fields');

      this.form.addEventListener('submit', (e) => [e.preventDefault(), this.findRule()]);
      console.info('ProxyForURL attached to form#proxy_for_url');
    }

    findRule() {
      const AsyncFunction = Object.getPrototypeOf(async function() {}).constructor;
      this._animate(true);

      try {
        const host = document.querySelector('#pac_host').value;
        const url = new URL(document.querySelector('#pac_url').value);
        this.log(null);
        this.log('FindProxyForURL', [url, host.length ? host : url.hostname], null);

        this.findProxyForURL = this.rulesInput.value
            .replace(/function\s+FindProxyForURL[^{]+{/, '')
            .replace(/\}\s*$/, '');

        for (let func of Object.keys(PAC_FUNCTIONS)) {
          const re = new RegExp('\\b' + func + '\\s*\\(', 'g');
          this.findProxyForURL = this.findProxyForURL.replace(re, 'await this._wrap("' + func + '", ');
        }

        this.findProxyForURL = this.findProxyForURL.replace(/\b(new\s|document\.|window\.|cookie\b)/, 'ILLEGAL');

        const fn = new AsyncFunction('url', 'host', this.findProxyForURL).bind(this);
        fn(url.toString(), host.length ? host : url.hostname).then(
          (rule) => {
            const lastCell = this.logEl.querySelector('tbody tr td:last-child');
            if (lastCell) lastCell.textContent = rule;
          },
          (err) => this.log(String(err), '', 'Error!'),
        ).finally(() => this._animate(false));
      } catch (err) {
        this._animate(false);
        let message = String(err);
        if (err.lineNumber) message += ' at line ' + err.lineNumber;
        if (err.columnNumber) message += ':' + err.columnNumber;
        this.log(message, '', 'Error!');
        throw err;
      }
    }

    log(msg, args, res) {
      const tbody = this.logEl.querySelector('tbody');
      if (msg === null) return tbody.innerHTML = '';
      const cells = [tbody.querySelectorAll('tr').length, msg, JSON.stringify(res)];

      if (args) {
        const prefix = PAC_FUNCTIONS[msg] === false ? '// ' : '';
        args = JSON.stringify(args).replace(/^\[/, '(').replace(/]$/, ')');
        cells[1] = prefix + msg + args;
      }

      const tr = document.createElement('tr');
      for (const content of cells) {
        const td = document.createElement('td');
        td.textContent = content;
        tr.appendChild(td);
      }

      tbody.appendChild(tr);
    }

    async alert() {
      return true;
    }

    async dateRange() {
      return true;
    }

    async dnsDomainIs(host, domain) {
      if (host === domain) return true;
      if (domain.startsWith('.')) return host.endsWith(domain);
      return host.endsWith('.' + domain);
    }

    async dnsDomainLevels(host) {
      const m = host.match(/\./g);
      return m ? m.length : 0;
    }

    async dnsResolve(host) {
      const body = new FormData();
      body.append('host', host);
      const res = await fetch('/v1/gethostbyname', {method: 'POST', body});
      const text = await res.text();
      if (res.status >= 500) throw 'dnsResolve() FAIL ' + (text || res.status);
      return text;
    }

    async isInNet(ip, net, mask) {
      // Ex: Turn 255.255.255.0 into 24
      if (mask.match(/\./)) mask = Math.round(mask.split('.').reduce((c, o) => c - Math.log2(256 - o), 32));

      const body = new FormData();
      body.append('ip', ip);
      body.append('net', net);
      body.append('mask', mask);
      const res = await fetch('/v1/is-in-net', {method: 'POST', body});
      const text = await res.text();
      if (res.status >= 500) throw 'isInNet() FAIL ' + (text || res.status);
      return parseInt(text, 10) ? true : false;
    }

    async isResolvable(host) {
      try {
        const ip = await this.dnsResolve(host);
        return ip && ip.length > 0 && ip.match(/[0-9.:]/);
      } catch (err) {
        return false;
      }
    }

    async isPlainHostName(str) {
      return str.match(/\./) ? false : true;
    }

    async localHostOrDomainIs(host, str) {
      if (str.startsWith('.')) return await this.dnsDomainIs(host, str);
      return host === str || (host.indexOf('.') === -1 && host === str.split('.')[0]);
    }

    async myIpAddress() {
      return this.myIpAddressInput.value || this.remoteAddress || '127.0.0.1';
    }

    async shExpMatch(host, shexp) {
      const pattern = shexp
        .replace(/[.+^${}()|[\]\\]/g, '\\$&')  // Escape regex special chars
        .replace(/\*/g, '.*')                   // * → match any chars
        .replace(/\?/g, '.');                   // ? → match single char
      return new RegExp('^' + pattern + '$', 'i').test(host);
    }

    async timeRange() {
      return true;
    }

    async weekdayRange() {
      return true;
    }

    _animate(start) {
      const method = start ? 'setAttribute' : 'removeAttribute';
      setTimeout(() => {
        this.form.querySelector('button')[method]('aria-busy', true);
      }, (start ? 1 : 350));
    }

    async _wrap(func) {
      const args = [].slice.call(arguments, 1);
      const res = await this[func].apply(this, args);
      this.log(func, args, res);
      return res;
    }
  }

  new ProxyForURL().attach(document)
})(window.thorsen || (window.thorsen = {}));
