/* global $ */

/* global getQueryParams */
window.getQueryParams = function getQueryParams() {
  return new URLSearchParams(window.location.search);
};
/* global sleep */
window.sleep = async function sleep(ms) {
  return new Promise((res) => setTimeout(res, ms));
};

/* global initTimer */
window.initTimer = function initTimer() {
  $("#timerWrapper").draggable({
    cancel: "input,textarea,button,select,option,svg,polygon,rect,#resizeHandle",
    containment: "window",
    iframeFix: true,
    scroll: false,
  });

  const wrapper = document.getElementById("timerWrapper");
  const handle = document.getElementById("resizeHandle");
  const baseFontSize = 3;
  const baseWidth = wrapper.offsetWidth;
  const baseBtnSize = 56;
  const baseIconSize = 24;
  let resizing = false;

  function applyScale(scale) {
    scale = Math.max(0.5, scale);
    wrapper.querySelectorAll("#timerText, #timerInput").forEach(el => {
      el.style.fontSize = (baseFontSize * scale) + "em";
    });
    wrapper.querySelectorAll("button").forEach(el => {
      el.style.width = Math.round(baseBtnSize * scale) + "px";
      el.style.height = Math.round(baseBtnSize * scale) + "px";
    });
    wrapper.querySelectorAll("button svg").forEach(el => {
      el.style.width = Math.round(baseIconSize * scale) + "px";
      el.style.height = Math.round(baseIconSize * scale) + "px";
    });
  }

  handle.addEventListener("mousedown", (e) => {
    e.preventDefault();
    e.stopPropagation();
    resizing = true;
    const startX = e.clientX;
    const startWidth = wrapper.offsetWidth;
    const cover = document.createElement("div");
    Object.assign(cover.style, {
      position: "fixed", inset: "0", width: "100%", height: "100%",
      zIndex: "999", cursor: "se-resize", margin: "0", padding: "0",
    });
    document.body.appendChild(cover);

    function onMove(e) {
      if (!resizing) return;
      const newWidth = startWidth + (e.clientX - startX);
      const scale = newWidth / baseWidth;
      applyScale(scale);
    }
    function onUp() {
      resizing = false;
      cover.remove();
      document.removeEventListener("mousemove", onMove);
      document.removeEventListener("mouseup", onUp);
    }
    document.addEventListener("mousemove", onMove);
    document.addEventListener("mouseup", onUp);
  });

  const query = getQueryParams();
  const queryTitle = query.get("title");
  if (queryTitle) {
    document.title = queryTitle;
  }
  const queryTime = query.get("time");
  if (queryTime) {
    $("#timerInput").val(queryTime);
  }
  if (!$("#clarificationsFrame").attr("src")) {
    let docURL = query.get("docURL");
    if (docURL) {
      docURL = new URL(docURL);
      if (docURL.pathname.endsWith("/view"))
        docURL.searchParams.set("rm", "minimal");
      $("#clarificationsFrame").attr("src", docURL.href);
    }
  }

  let interval = null;
  let running = false;
  let remainingMs = 0;
  let endTime = 0;
  let durationMs = 0;

  function setPlayIcon() {
    $("#playIcon").removeClass("hidden");
    $("#pauseIcon").addClass("hidden");
    $("#timerPlay")
      .removeClass("bg-gray-500 hover:bg-gray-600")
      .addClass("bg-green-500 hover:bg-green-600");
  }

  function setPauseIcon() {
    $("#playIcon").addClass("hidden");
    $("#pauseIcon").removeClass("hidden");
    $("#timerPlay")
      .removeClass("bg-green-500 hover:bg-green-600")
      .addClass("bg-gray-500 hover:bg-gray-600");
  }

  function enableReset() {
    $("#timerReset")
      .prop("disabled", false)
      .removeClass("bg-gray-450 cursor-not-allowed")
      .addClass("bg-red-500 hover:bg-red-600 cursor-pointer");
  }

  function disableReset() {
    $("#timerReset")
      .prop("disabled", true)
      .removeClass("bg-red-500 hover:bg-red-600 cursor-pointer")
      .addClass("bg-gray-450 cursor-not-allowed");
  }

  function resetTimer() {
    clearInterval(interval);
    interval = null;
    running = false;
    remainingMs = 0;
    endTime = 0;
    durationMs = 0;

    $("#timerText").hide();
    $("#timerInput").show();
    $("#timerWrapper").removeClass("expired blink");
    setPlayIcon();
    disableReset();
  }

  function refreshTimer() {
    let timeLeft = Math.min(endTime - Date.now(), durationMs);

    if (timeLeft <= 0) {
      $("#timerText").text("0:00:00");
      $("#timerWrapper").addClass("expired");
      running = false;
      setPlayIcon();

      clearInterval(interval);
      interval = setInterval(() => {
        $("#timerWrapper").toggleClass("blink");
      }, 500);

      return;
    }

    const hoursLeft = Math.floor(timeLeft / (1000 * 60 * 60));
    timeLeft -= 1000 * 60 * 60 * hoursLeft;
    const minutesLeft = Math.floor(timeLeft / (1000 * 60));
    timeLeft -= 1000 * 60 * minutesLeft;
    const secondsLeft = Math.floor(timeLeft / 1000);

    const timeStr = `${hoursLeft}:${`00${String(minutesLeft)}`.slice(
      -2
    )}:${`00${String(secondsLeft)}`.slice(-2)}`;
    $("#timerText").text(timeStr);
  }

  function startTimer() {
    const inputVal = $("#timerInput").val();
    if (inputVal.startsWith("http")) {
      const newURL = new URL(window.location.href);
      newURL.searchParams.set("docURL", inputVal);
      window.location.href = newURL.href;
      return;
    }

    const durationMinutes = parseFloat(inputVal);
    if (Number.isNaN(durationMinutes)) {
      alert(`Invalid duration: ${inputVal}`); // eslint-disable-line no-alert
      return;
    }

    durationMs = 1000 * 60 * durationMinutes;
    remainingMs = durationMs;
    resumeTimer();
  }

  function resumeTimer() {
    $("#timerInput").hide();
    $("#timerText").show();
    $("#timerWrapper").removeClass("expired blink");
    setPauseIcon();
    enableReset();

    running = true;
    endTime = Date.now() + remainingMs;

    clearInterval(interval);
    interval = setInterval(refreshTimer, 400);
    refreshTimer();
  }

  function pauseTimer() {
    remainingMs = Math.max(0, endTime - Date.now());
    running = false;
    clearInterval(interval);
    interval = null;
    setPlayIcon();
  }

  $("#timerPlay").click(() => {
    if ($("#timerWrapper").hasClass("expired")) {
      resetTimer();
      return;
    }
    if (running) {
      pauseTimer();
    } else if (durationMs > 0) {
      resumeTimer();
    } else {
      startTimer();
    }
  });

  $("#timerReset").click(() => {
    resetTimer();
  });
};
$(document).ready(initTimer);
