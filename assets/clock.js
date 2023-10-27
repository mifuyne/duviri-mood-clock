import init, * as wasm from "../pkg/duviri_mood_clock_wasm.js";
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
  const curr_mood = JSON.parse(wasm.get_current_mood())
  let current_mood_el = document.querySelector("#current-mood")
  let current_time_el = document.querySelector("#current-time")

  console.log(curr_mood)

  let current_time = datetime_fmt.format(new Date(curr_mood.time))
  current_mood_el.innerText = curr_mood.mood
  current_time_el.innerText = `${current_time}`

  // Future moods
  if ("content" in document.createElement("template")) {
    const next_moods = JSON.parse(wasm.get_next_mood((Date.now() / 1000).toFixed(), 3));
    const tbody = document.getElementById("next").querySelector("table > tbody")
    const template = document.querySelector("template#mood-row")

    console.log(next_moods)

    for (let mood of next_moods) {
      const clone = template.content.cloneNode(true)
      let td = clone.querySelectorAll("td")

      td[0].textContent = mood.mood
      td[1].textContent = datetime_fmt.format(new Date(mood.time))

      tbody.appendChild(clone)
    }
  }
});