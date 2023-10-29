import init, * as wasm from "/pkg/duviri_mood_clock_wasm.js";
init().then(() => {
  // Current mood
  const datetime_fmt = new Intl.DateTimeFormat("en", {
    year: "numeric",
    month: "long",
    day: "numeric",
    hour: "numeric",
    minute: "numeric",
    // timeZoneName: "short",
  })
  const body_el = document.querySelector("body")
  
  let current_mood_el = document.getElementById("current-mood")
  let current_time_el = document.getElementById("current-time")

  loadCurrentMood(body_el, current_mood_el, current_time_el, datetime_fmt)

  document.getElementById("btn-refresh").addEventListener("click", (event) => {
    loadCurrentMood(body_el, current_mood_el, current_time_el, datetime_fmt)
  })

  // Future moods
  if ("content" in document.createElement("template")) {
    const next_moods = JSON.parse(wasm.get_next_mood((Date.now() / 1000).toFixed(), 3));
    const tbody = document.getElementById("next").querySelector("table > tbody")
    const template = document.querySelector("template#mood-row")

    for (let mood of next_moods) {
      const clone = template.content.cloneNode(true)
      let td = clone.querySelectorAll("td")

      td[0].textContent = mood.mood
      td[1].textContent = datetime_fmt.format(new Date(mood.time))

      tbody.appendChild(clone)
    }
  }
});

function loadCurrentMood(body, mood_el, time_el, dt_fmt) {
  mood_el.innerText = ``
  time_el.innerText = ``

  let curr_mood = JSON.parse(wasm.get_current_mood())
  let current_time = dt_fmt.format(new Date(curr_mood.time))

  mood_el.innerText = curr_mood.mood
  time_el.innerText = `${current_time}`

  let mood_class = curr_mood.mood.toLowerCase()
  if (!body.classList.contains(mood_class)) {
    body.classList.add(mood_class)
  }
  
}