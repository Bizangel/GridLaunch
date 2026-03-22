// Adapted from: https://github.com/partydeck/partydeck/blob/main/res/splitscreen_kwin.js
const x = [
  [],
  [0],
  [0, 0.5],
  [0, 0, 0.5],
  [0, 0.5, 0, 0.5]
]

const y = [
  [],
  [0],
  [0, 0],
  [0, 0.5, 0.5],
  [0, 0, 0.5, 0.5]
]

const width = [
  [],
  [1],
  [0.5, 0.5],
  [1, 0.5, 0.5],
  [0.5, 0.5, 0.5, 0.5]
]

const height = [
  [],
  [1],
  [1, 1],
  [0.5, 0.5, 0.5],
  [0.5, 0.5, 0.5, 0.5]
]

function getGamescopeWindows() {
  return workspace.windowList().filter(e => e.resourceClass == "gamescope");
}

function numGamescopeClientsInOutput(output) {
    return getGamescopeWindows().filter(e => e.output.name == output.name).length;
}

function gamescopeSetKeepAbove() {
  var gamescopeClients = getGamescopeWindows();
  for (var i = 0; i < gamescopeClients.length; i++) {
    if (
      workspace.activeWindow.resourceClass == "gamescope"
    ) {
      gamescopeClients[i].keepAbove = true;
    } else {
      gamescopeClients[i].keepAbove = false;
    }
  }
}

function onWindowsChange() {
  var gamescopeWindows = getGamescopeWindows();

  var screenMap = new Map();
  var screens = workspace.screens;
  for (var j = 0; j < screens.length; j++) {
    screenMap.set(screens[j], 0);
  }

  for (var i = 0; i < gamescopeWindows.length; i++) {
    var monitor = gamescopeWindows[i].output;
    var monitorX = monitor.geometry.x;
    var monitorY = monitor.geometry.y;
    var monitorWidth = monitor.geometry.width;
    var monitorHeight = monitor.geometry.height;

    var playerCount = numGamescopeClientsInOutput(monitor);
    var playerIndex = screenMap.get(monitor);
    screenMap.set(monitor, playerIndex + 1);

    gamescopeWindows[i].noBorder = true;
    gamescopeWindows[i].frameGeometry = {
      x: monitorX + x[playerCount][playerIndex] * monitorWidth,
      y: monitorY + y[playerCount][playerIndex] * monitorHeight,
      width: monitorWidth * width[playerCount][playerIndex],
      height: monitorHeight * height[playerCount][playerIndex],
    };
  }

    gamescopeSetKeepAbove();
}

workspace.windowAdded.connect(onWindowsChange);
workspace.windowRemoved.connect(onWindowsChange);
workspace.windowActivated.connect(gamescopeSetKeepAbove);