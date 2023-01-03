function handleSubmit(e) {
  e.preventDefault();
  // console.log(e);
  const data = new FormData(e.target);
  const frets = [
    data.get("frets-low-e") || -1,
    data.get("frets-a") || -1,
    data.get("frets-d") || -1,
    data.get("frets-g") || -1,
    data.get("frets-b") || -1,
    data.get("frets-high-e") || -1,
  ].join(",");
  const fingers = [
    data.get("fingers-low-e") || "x",
    data.get("fingers-a") || "x",
    data.get("fingers-d") || "x",
    data.get("fingers-g") || "x",
    data.get("fingers-b") || "x",
    data.get("fingers-high-e") || "x",
  ].join(",");
  const transformed = new URLSearchParams();

  transformed.append("frets", frets);
  transformed.append("fingers", fingers);
  transformed.append("title", data.get("title") || "");

  const headers = new Headers();
  headers.append("Content-Type", "application/x-www-form-urlencoded");
  fetch(process.env.API_URL, {
    method: "POST",
    headers,
    body: transformed,
  })
    .then((response) => {
      response.json().then(({ path }) => {
        showChordImage(path);
      });
    })
    .catch((e) => console.error("oh no", e));
}

function showChordImage(path) {
  const chord = document.querySelector("#chord");
  chord.innerHTML = "";
  let img = document.createElement("img");
  img.src = `${process.env.IMAGES_URL}/${path}.svg`;
  // img.width = "320";
  // img.height = "388";
  img.alt = `Guitar chord diagram for ${
    document.querySelector("#title").value
  }`;
  chord.appendChild(img);
}

function applyPreset(preset) {
  document.querySelector("#title").value = preset.title;
  document.querySelector("#frets-low-e").value = preset.frets[0];
  document.querySelector("#frets-a").value = preset.frets[1];
  document.querySelector("#frets-d").value = preset.frets[2];
  document.querySelector("#frets-g").value = preset.frets[3];
  document.querySelector("#frets-b").value = preset.frets[4];
  document.querySelector("#frets-high-e").value = preset.frets[5];

  document.querySelector("#fingers-low-e").value = preset.fingers[0];
  document.querySelector("#fingers-a").value = preset.fingers[1];
  document.querySelector("#fingers-d").value = preset.fingers[2];
  document.querySelector("#fingers-g").value = preset.fingers[3];
  document.querySelector("#fingers-b").value = preset.fingers[4];
  document.querySelector("#fingers-high-e").value = preset.fingers[5];

  showChordImage(preset.image);
}

function createPresets(presets) {
  presets.forEach((preset) => {
    const presetsSection = document.querySelector(".presets");
    const btn = document.createElement("button");
    btn.classList.add("button");
    btn.textContent = preset.title;
    btn.addEventListener("click", () => applyPreset(preset));
    presetsSection.appendChild(btn);
  });
}

const presets = [
  {
    title: "E",
    frets: ["0", "2", "2", "1", "0", "0"],
    fingers: ["0", "2", "3", "1", "0", "0"],
    image: "18436534002643003894",
  },
  {
    title: "D",
    frets: ["x", "x", "0", "2", "3", "2"],
    fingers: ["x", "x", "0", "2", "3", "1"],
    image: "4095730029079104823",
  },
  {
    title: "A",
    frets: ["x", "0", "2", "2", "2", "0"],
    fingers: ["x", "0", "2", "1", "3", "0"],
    image: "6374786531096975228",
  },
  {
    title: "G",
    frets: ["3", "2", "0", "0", "0", "3"],
    fingers: ["2", "1", "0", "0", "0", "3"],
    image: "8535511527932517360",
  },
  {
    title: "C",
    frets: ["x", "3", "2", "0", "1", "0"],
    fingers: ["x", "3", "2", "0", "1", "0"],
    image: "452844100226506193",
  },
  {
    title: "Hendrix",
    frets: ["x", "7", "6", "7", "8", "x"],
    fingers: ["x", "2", "1", "3", "4", "x"],
    image: "13217194300744275703",
  },
  {
    title: "Bond",
    frets: ["0", "10", "9", "8", "7", "x"],
    fingers: ["0", "4", "3", "2", "1", "x"],
    image: "12540277254987366366",
  },
  {
    title: "C°7",
    frets: ["x", "3", "4", "2", "3", "x"],
    fingers: ["x", "2", "3", "1", "4", "x"],
    image: "15615698213659243213",
  },
  {
    title: "D7",
    frets: ["10", "12", "10", "11", "10", "10"],
    fingers: ["1", "3", "1", "2", "1", "1"],
    image: "13518970828834701382",
  },
  {
    title: "A7",
    frets: ["x", "x", "9", "11", "10", "x"],
    fingers: ["x", "x", "1", "3", "2", "x"],
    image: "3854455750811480831",
  },
  {
    title: "E9",
    frets: ["x", "7", "6", "7", "7", "7"],
    fingers: ["x", "2", "1", "3", "3", "3"],
    image: "13724104169966017016",
  },
  {
    title: "E♭m7",
    frets: ["x", "6", "8", "6", "7", "6"],
    fingers: ["x", "1", "3", "1", "2", "1"],
    image: "3373453791652677623",
  },
];

const form = document.querySelector("#generate");
form.addEventListener("submit", handleSubmit);

createPresets(presets);

const buttons = document.querySelectorAll(".presets .button");
buttons[Math.floor(Math.random() * buttons.length)].click();

const symbols = ["♭", "♯", "♮", "°", "+"];
const title = document.querySelector("#title");
symbols.forEach((symbol) => {
  const symbolSection = document.querySelector(".symbols");
  const btn = document.createElement("button");
  btn.classList.add("button--secondary");
  btn.type = "button";
  btn.textContent = symbol;
  btn.addEventListener(
    "click",
    () => (title.value = `${title.value}${symbol}`)
  );
  symbolSection.appendChild(btn);
});
