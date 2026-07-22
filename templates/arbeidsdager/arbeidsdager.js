((thorsen) => {
  const ONE_DAY = 1000 * 60 * 60 * 24;
  const MONDAY = 1;
  const SATURDAY = 6;

  const el = ($el, sel) => $el.querySelector(sel);

  const calculateTrappedDays = ($form, from, to) => {
    const nextWeek = new Date(from.valueOf() + ONE_DAY * 7);
    if (nextWeek.valueOf() < to.valueOf()) to = nextWeek;

    let n = 0;
    while (from.valueOf() <= to.valueOf()) {
      const ymd = from.toISOString().split("T")[0];
      const day = from.getDay();
      if (!el(document, `td[data-date="${ymd}"]`) && day >= MONDAY && day <= SATURDAY) n++;
      from.setDate(from.getDate() + 1);
    }

    return n;
  };

  const str2date = (str) => {
    const ymd = str.trim().split("-").map(v => v.replace(/^0+/, "")).map(v => parseInt(v, 10));
    return new Date(ymd[0], ymd[1] - 1, ymd[2]);
  };

  thorsen.calculateWorkingDays = (evt) => {
    if (evt) evt.preventDefault();
    const $form = document.querySelector("form[action=\"/arbeidsdager\"]");

    const to = str2date(el($form, "input[name=to]").value);
    let from = new Date(to.getFullYear(), 0, 1);

    const result = {holidays: 0, weekends: 0};
    result.days = ((to.valueOf() - from.valueOf()) / ONE_DAY) + 1;

    let easter;
    for (const $tr of document.querySelectorAll("#holidays-table [data-kind]")) {
      const $cols = $tr.querySelectorAll("td");
      const date = str2date($cols[0].textContent);
      const month = date.getMonth() + 1;
      if (date.valueOf() > to.valueOf()) {
        break;
      }

      if ($cols[1].textContent.match(/Palme.*dag/i)) {
        easter = date;
      }

      const kind = $tr.dataset.kind;
      if (kind === "saturday" || kind === "sunday") {
        result.weekends++;
      } else {
        result.holidays++;
      }
    }

    result.christmas = calculateTrappedDays($form, new Date(to.getFullYear(), 12 - 1, 24), to);
    result.easter = easter ? calculateTrappedDays($form, easter, to) : 0;

    for (const key of Object.keys(result)) {
      el($form, `.total-${key}`).textContent = result[key];
    }

    let workingDays = result.days;
    workingDays -= parseInt(el($form, "[name=vacation_days]").value);
    if (el($form, "[name=christmas]").checked) workingDays -= result.christmas;
    if (el($form, "[name=easter]").checked) workingDays -= result.easter;
    if (el($form, "[name=holidays]").checked) workingDays -= result.holidays;
    if (el($form, "[name=weekends]").checked) workingDays -= result.weekends;

    el($form, `.total-working-days`).textContent = workingDays;
    el($form, `.total-working-hours`).textContent = workingDays * 7.5;

    const url = new URL(location.href);
    url.search = new URLSearchParams(new FormData($form)).toString()
    history.replaceState({total: workingDays}, null, url.toString());
  };
})(window.thorsen || (window.thorsen = {}));
