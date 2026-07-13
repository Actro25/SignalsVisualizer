const startButton = document.getElementById("ws_start");
const stopButton = document.getElementById("ws_stop");
const loadPlotButton = document.getElementById("load_plot");

const showPlotDiv = document.getElementById("show");

let ws = null;
let uplot = null;

startButton.addEventListener('click', function () {
    if(ws){
        console.log("Active.");
        return;
    }

    ws = new WebSocket("/ws");

    ws.onmessage = (event) => {
        try {
            const signal = JSON.parse(event.data);
            console.log(signal);
            uplot.setData([signal.x, signal.y]);
        }
        catch (err){
            console.log("Error JSON serializing: ", err);
        }
    }

    ws.onerror = (error) => {
        console.log("Error WebSocket: ", error);
    }
})

stopButton.addEventListener('click', function () {
    ws.close();
    ws = null;
})

loadPlotButton.addEventListener('click', function () {
    const data = [
        [0, 1, 2, 3, 4, 5],
        [10, 15, 13, 17, 20, 18],
    ];

    const opts = {
        width: 800,
        height: 400,
        series: [
            {},
            {
                label: "Значення",
                stroke: "blue",
                width: 2,
            },
        ],
    };
    uplot = new uPlot(opts, data, showPlotDiv);
})