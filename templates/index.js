const ratings = document.getElementsByClassName("rating");
const button = document.getElementById("submit")

for (let i=0; i < ratings.length; i++) {
    let slider = ratings[i];

    slider.children[1].addEventListener("input", e => {
        slider.children[0].children[1].innerHTML = parseFloat(slider.children[1].value).toFixed(1);
    });
    
    slider.children[0].children[1].innerHTML = parseFloat(slider.children[0].children[1].innerHTML).toFixed(1);
}

document.getElementById("nav-icon")
    .addEventListener("click", () => {
        document.location.href = "/"
    })

button.addEventListener("click", async () => {
    console.log("Hello, World!");

    let user_ratings = {};

    // Collect current results
    for (let i=0; i < ratings.length; i++) {
        let rating = ratings[i];
        let top = rating.children[0];

        let name = top.children[0].innerHTML;
        let skill_rating = parseFloat(top.children[1].innerHTML);

        user_ratings[name] = skill_rating;
    }

    console.log(user_ratings);

    button.innerHTML = "...";

    fetch("/teams/ratings", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({ratings: user_ratings})
    })
    .then(async resp => {
        switch (resp.ok) {
            case true:
                button.style.backgroundColor = "#3A4239"
                button.innerHTML = "Success!"
                await new Promise(r => setTimeout(r, 100));
                break;
            
            case false:
                button.style.backgroundColor = "#423939"
                button.innerHTML = "Error"
                await new Promise(r => setTimeout(r, 100));
                break;
        }
        button.style.backgroundColor = "#1C1C1C"
    });

    await new Promise(r => setTimeout(r, 1000));
    button.innerHTML = "Submit"
});