const DAY = 86400, HOUR = 3600, MINUTE = 60;
const on = (el, type, cb) => el.addEventListener(type, e => { e.preventDefault(); cb(e); });
const now = () => parseInt(new Date().valueOf() / 1000, 10);

function q(parentEl, sel, cb) {
  if (!cb) [parentEl, sel, cb] = [document, parentEl, sel];
  const els = sel == ':children' ? parentEl.children : parentEl.querySelectorAll(sel);
  for (let i = 0; i < els.length; i++) cb(els[i], i);
}

class Timekeeper {
  constructor() {
    this.inputNames = ['d', 'h', 'm', 's'];
    this.tracked = {};
  }

  attach(formEl) {
    this.baseUrl = formEl.action.split('#')[0].split('?')[0].replace(/\/+$/, '');
    this.formEl = formEl;
    q(this.formEl, '.alarm-player', el => (this.alarmPlayer = el));

    this.inputNames.forEach(name => {
      q(formEl, '[name="' + name + '"]', input => {
        this._renderInput(input, input.value);
        input.addEventListener('blur', e => this._onBlur(input, e));
        input.addEventListener('keydown', e => this._onKeydown(input, e));
      });
    });

    this.alarmPlayer.volume = 0.4; // The alarm sound is crazy loud
    this._renderAlarm();
    this._startOrStop({});

    on(this.formEl, 'submit', () => this.start());
    window.addEventListener('popstate', e => this._startOrStop(e));

    q('input[id=alarm]', el => on(el, 'change', e => t.toggleAlarm(e)));
    q('a[href^="#start:"]', el => on(el, 'click', () => t.start(el.href.split('#start:')[1])));
    q('a[href^="#stop"]', el => on(el, 'click', () => t.stop()));
    q('a[href^="#edit"]', el => on(el, 'click', () => this.formEl.m.focus()));
    q('a[href="#edit"]', el => el.focus());

    return this;
  }

  alarmActive() {
    return localStorage.getItem('timer_alarm') != 'off';
  }

  inputsToSeconds() {
    const formEl = this.formEl;
    return parseInt(formEl.d.value.replace(/^0+/, '') || 0, 10) * DAY
      + parseInt(formEl.h.value.replace(/^0+/, '') || 0, 10) * HOUR
      + parseInt(formEl.m.value.replace(/^0+/, '') || 0, 10) * MINUTE
      + parseInt(formEl.s.value.replace(/^0+/, '') || 0, 10);
  }

  start(seconds, epoch = now()) {
    seconds = parseInt(seconds, 10) || this.inputsToSeconds();

    if (seconds) {
      history.replaceState({}, document.title, `${this.baseUrl}?${seconds}/${epoch}`);
      window.scrollTo(0, 0);
      setTimeout(() => q('[href="#stop"]', el => el.focus()), 100);
    }

    this._startOrStop({});
  }

  stop() {
    if (this.tid) clearInterval(this.tid);
    if (location.href.match(/(\d+)\/(\d+)/)) history.pushState({}, document.title, this.baseUrl);
    this._state('editing');
    let total = 0;
    for (const name of this.inputNames) {
      const val = localStorage.getItem('timer_' + name) || '0';
      total += val;
      this._renderInput(this.formEl[name], val);
    }

    if (total == 0) this._renderInput(this.formEl['m'], 5);
  }

  toggleAlarm() {
    localStorage.setItem('timer_alarm', this.alarmActive() ? 'off' : 'on');
  }

  _onBlur(input, e) {
    if (!input.value.length) this._renderInput(input, '0');
    localStorage.setItem('timer_' + input.name, input.value);
  }

  _onKeydown(input, e) {
    if (input.disabled) return;
    if (e.altKey || e.ctrlKey || e.metaKey) return; // Do not want to capture special keys
    if (e.keyCode == 13) return [e.preventDefault(), this.start()];

    const num = (e.keyCode || e.which) - 48; // Convert keycode to {0..9}
    if (num < 0 || num > 9) return; // Tab, delete, ... is not a number

    e.preventDefault();
    const max = input.getAttribute('max');
    const v = input.value.length >= max.length ? num : parseInt(input.value + '' + num, 10);
    this._renderInput(input, v > max ? max : v < 0 ? 0 : v);
  }

  _renderAlarm() {
    q(this.formEl, '[id=alarm]', el => el.checked = this.alarmActive());
  }

  _renderCountdown() {
    const formEl = this.formEl;
    const left = [0, 0, 0, this.ends - now()];
    const title = [];

    if (left[3] <= 0) {
      if (this.tid) clearInterval(this.tid);
      setTimeout(() => this.alarmPlayer.pause(), 4000);
      if (this.alarmActive()) this.alarmPlayer.play();
      this.inputNames.forEach(name => this._renderInput(formEl[name], '0'));
      this._state('expired');
      return;
    }

    left[0] = parseInt(left[3] / DAY, 10);
    left[3] -= left[0] * DAY;
    left[1] = parseInt(left[3] / HOUR, 10);
    left[3] -= left[1] * HOUR;
    left[2] = parseInt(left[3] / MINUTE, 10);
    left[3] -= left[2] * MINUTE;

    for (let i = 0; i < left.length; i++) {
      if (left[i] || title.length) title.push(left[i] + this.inputNames[i]);
    }

    document.title = title.join(' ');
    this._renderInput(formEl.d, left[0]);
    this._renderInput(formEl.h, left[1]);
    this._renderInput(formEl.m, left[2]);
    this._renderInput(formEl.s, left[3]);
  }

  _renderInput(input, val) {
    input.value = String(val);
  }

  _startOrStop() {
    if (this.tid) clearInterval(this.tid);

    const [_, seconds, epoch] = location.href.match(/(\d+)\/(\d+)/) || [];
    this.seconds = parseInt(seconds, 10);
    this.ends = parseInt(epoch, 10) + this.seconds;
    if (!this.ends) return this.stop();
    if (this.ends - now() <= 0) return this._state('expired');

    this._state('running');
    this._renderCountdown();
    this.tid = setInterval(() => this._renderCountdown(), 1000);
  }

  _state(state) {
    if (state == 'edit') q('[property="og:title"]', el => (document.title = el.content));
    if (state == 'expired') q('.expired-text', el => (document.title = el.textContent));
    q(this.formEl, 'input', el => (el.disabled = state == 'countdown'));
    q(this.formEl, '[class*="show-when-"]', el => el.setAttribute('hidden', 'hidden'));
    q(this.formEl, '.show-when-' + state, el => el.removeAttribute('hidden'));
  }
}

const t = new Timekeeper().attach(document.querySelector('form'));
