const input = document.getElementById("searchbar");
const auto_list = document.getElementById("auto_list");
const result_list = document.getElementById("result_list");
const button = document.getElementById("search_button");

input.addEventListener("input", async (e) => {
    const value = e.target.value;

    const res = await fetch(`/api/search?q=${encodeURIComponent(value)}`);
    const data = await res.json();

    auto_list.innerHTML = "";

    for (const [title, link] of data) {
        const line = document.createElement("li");
        line.innerHTML = "<a href='"+link+"'>"+title+"</a>";
        auto_list.appendChild(line);
    }
});

button.addEventListener("click", async (e) => {
    const value = input.value;

    const res = await fetch(`/api/result?q=${encodeURIComponent(value)}`);
    const data = await res.json();

    result_list.innerHTML = "";
    console.log(data);
    for (const [title, link, scores] of data) {
        const line = document.createElement("li");
        line.innerHTML = "<a href='"+link+"'>"+"Article: "+title+"  | Score: "+scores.toFixed(4)+"</a>";
        result_list.appendChild(line);
    }

});

