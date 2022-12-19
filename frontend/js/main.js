function handleSubmit(e) {
  e.preventDefault();
  // console.log(e);
  const data = new FormData(e.target);
  console.log(Object.fromEntries(data));
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

  console.log(Object.fromEntries(transformed));

  // const prod = "https://chordgenerator.xyz/api/";
  // const dev = "http://localhost:4041";
  const headers = new Headers();
  headers.append("Content-Type", "application/x-www-form-urlencoded");
  fetch(process.env.API_URL, {
    method: "POST",
    headers,
    body: transformed,
  })
    .then((response) => {
      response.json().then(({ path }) => {
        console.log(path);
        const chord = document.querySelector("#chord");
        chord.innerHTML = "";
        let img = document.createElement("img");
        img.src = `${process.env.API_URL}/images/${path}.png`;
        chord.appendChild(img);
      });
    })
    .catch((e) => console.error("oh no", e));
}

const form = document.querySelector("#generate");
form.addEventListener("submit", handleSubmit);
