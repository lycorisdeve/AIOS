const state = {
  intentTitle: document.querySelector("#intentTitle"),
  intentSummary: document.querySelector("#intentSummary"),
  intentInput: document.querySelector("#intentInput"),
  selectedApp: document.querySelector("#selectedApp"),
  appIcon: document.querySelector("#appIcon"),
  appTitle: document.querySelector("#appTitle"),
  appSurface: document.querySelector("#appSurface"),
  policyState: document.querySelector("#policyState"),
  policyApp: document.querySelector("#policyApp"),
  policyAction: document.querySelector("#policyAction"),
  executionState: document.querySelector("#executionState"),
  executionSteps: document.querySelector("#executionSteps"),
  floatingIntentBar: document.querySelector("#floatingIntentBar"),
  dragHandle: document.querySelector(".drag-handle")
};

const scenarios = {
  chat: {
    app: "微信",
    icon: "微",
    action: "Send Message",
    route: "Chat Invocation",
    summary: "已识别为消息发送任务，正在调用聊天应用并生成待确认消息。",
    steps: ["理解目标", "定位联系人", "调用聊天应用", "等待用户确认"],
    render({ contact = "张三", message = "今晚吃瓦香鸡。" } = {}) {
      return `
        <div class="chat-thread">
          <div class="chat-row incoming">
            <span class="avatar">${contact.slice(0, 1)}</span>
            <p>晚上吃什么？</p>
          </div>
          <div class="chat-row outgoing draft">
            <p id="draftMessage">${message}</p>
            <span class="avatar">我</span>
          </div>
        </div>
        <div class="confirmation-card">
          <div>
            <span class="confirm-label">Message Preview</span>
            <strong id="confirmText">发送给 ${contact}：${message}</strong>
          </div>
          <div class="confirm-actions">
            <button class="secondary-action" type="button">修改</button>
            <button id="confirmButton" class="primary-action" type="button">确认发送</button>
          </div>
        </div>
      `;
    }
  },
  calendar: {
    app: "日历",
    icon: "日",
    action: "Create Event",
    route: "Calendar Invocation",
    summary: "已识别为日程任务，正在调用日历应用生成待确认事件。",
    steps: ["理解时间", "检查日历", "创建事件草稿", "等待用户确认"],
    render() {
      return `
        <div class="calendar-surface">
          <article class="app-card">
            <strong>明天 09:00</strong>
            <span>销售复盘会议 · 会议室 A · 预计 45 分钟</span>
          </article>
          <div class="confirmation-card">
            <div>
              <span class="confirm-label">Event Preview</span>
              <strong>创建日程：明天 09:00 销售复盘会议</strong>
            </div>
            <div class="confirm-actions">
              <button class="secondary-action" type="button">修改</button>
              <button class="primary-action" type="button">确认创建</button>
            </div>
          </div>
        </div>
      `;
    }
  },
  files: {
    app: "文件",
    icon: "文",
    action: "Open File",
    route: "File Invocation",
    summary: "已识别为文件访问任务，正在打开相关文档。",
    steps: ["理解文件意图", "搜索本地索引", "打开文档", "显示结果"],
    render() {
      return `
        <div class="file-surface">
          <article class="app-card">
            <strong>AIOS Architecture.md</strong>
            <span>Goal Kernel、Agent Runtime、Tool ABI、Policy Kernel</span>
          </article>
          <article class="app-card">
            <strong>UI_REFERENCE.md</strong>
            <span>玻璃态 Shell、应用调用、确认界面</span>
          </article>
        </div>
      `;
    }
  },
  browser: {
    app: "浏览器",
    icon: "浏",
    action: "Open Web",
    route: "Browser Invocation",
    summary: "已识别为网页访问任务，正在调用浏览器打开目标页面。",
    steps: ["理解网址", "检查网络", "打开浏览器", "显示网页"],
    render() {
      return `
        <div class="browser-surface">
          <article class="app-card">
            <strong>aios.local/workspace</strong>
            <span>系统工作区已打开。</span>
          </article>
        </div>
      `;
    }
  }
};

function extractChatIntent(text) {
  const clean = text.trim();
  const tellMatch = clean.match(/告诉(.+?)[，, ](.+)/);
  const sendMatch = clean.match(/给(.+?)(发|发送)(.+)/);

  if (tellMatch) {
    return {
      contact: tellMatch[1].trim(),
      message: punctuate(tellMatch[2].trim())
    };
  }

  if (sendMatch) {
    return {
      contact: sendMatch[1].trim(),
      message: punctuate(sendMatch[3].trim())
    };
  }

  return {
    contact: "张三",
    message: "今晚吃瓦香鸡。"
  };
}

function punctuate(value) {
  return /[。！？.!?]$/.test(value) ? value : `${value}。`;
}

function renderSteps(steps) {
  state.executionSteps.innerHTML = "";
  steps.forEach((step, index) => {
    const item = document.createElement("li");
    item.textContent = step;
    if (index < steps.length - 1) item.className = "done";
    if (index === steps.length - 1) item.className = "running";
    state.executionSteps.appendChild(item);
  });
}

function openScenario(name, options = {}) {
  const scenario = scenarios[name];
  if (!scenario) return;

  state.intentTitle.textContent = options.intent || state.intentInput.value || "告诉张三，今晚吃瓦香鸡";
  state.intentSummary.textContent = scenario.summary;
  state.selectedApp.textContent = scenario.route;
  state.appIcon.textContent = scenario.icon;
  state.appTitle.textContent = scenario.app;
  state.policyApp.textContent = scenario.app;
  state.policyAction.textContent = scenario.action;
  state.policyState.textContent = name === "chat" || name === "calendar" ? "等待确认" : "已打开";
  state.executionState.textContent = name === "chat" || name === "calendar" ? "Preview" : "Done";
  state.appSurface.className = `app-surface ${name}-surface`;
  state.appSurface.innerHTML = scenario.render(options);
  renderSteps(scenario.steps);

  const confirmButton = document.querySelector(".primary-action");
  if (confirmButton) {
    confirmButton.addEventListener("click", () => {
      state.policyState.textContent = "已执行";
      state.executionState.textContent = "Done";
      const steps = Array.from(state.executionSteps.querySelectorAll("li"));
      steps.forEach((item) => {
        item.className = "done";
      });
      confirmButton.textContent = "已完成";
    });
  }
}

document.querySelector(".intent-bar").addEventListener("submit", (event) => {
  event.preventDefault();
  const text = state.intentInput.value.trim();
  if (!text) {
    openScenario("chat", { intent: "告诉张三，今晚吃瓦香鸡", ...extractChatIntent("") });
    return;
  }

  if (/告诉|发消息|发送|微信|给.+发/.test(text)) {
    openScenario("chat", { intent: text, ...extractChatIntent(text) });
  } else if (/日程|会议|提醒|明天|今晚.*点/.test(text)) {
    openScenario("calendar", { intent: text });
  } else if (/打开|文件|文档/.test(text)) {
    openScenario("files", { intent: text });
  } else if (/网页|浏览器|网站/.test(text)) {
    openScenario("browser", { intent: text });
  } else {
    openScenario("chat", { intent: text, contact: "张三", message: punctuate(text) });
  }
});

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}

function setFloatingBarPosition(left, top) {
  const bar = state.floatingIntentBar;
  const rect = bar.getBoundingClientRect();
  const nextLeft = clamp(left, 12, window.innerWidth - rect.width - 12);
  const nextTop = clamp(top, 58, window.innerHeight - rect.height - 12);

  bar.style.left = `${nextLeft}px`;
  bar.style.top = `${nextTop}px`;
  bar.style.right = "auto";
  bar.style.bottom = "auto";
  bar.style.transform = "none";
  localStorage.setItem("aios.floatingIntentBar", JSON.stringify({ left: nextLeft, top: nextTop }));
}

function restoreFloatingBarPosition() {
  const saved = localStorage.getItem("aios.floatingIntentBar");
  if (!saved) return;

  try {
    const position = JSON.parse(saved);
    if (Number.isFinite(position.left) && Number.isFinite(position.top)) {
      setFloatingBarPosition(position.left, position.top);
    }
  } catch {
    localStorage.removeItem("aios.floatingIntentBar");
  }
}

function enableFloatingDrag() {
  let dragState = null;

  state.dragHandle.addEventListener("pointerdown", (event) => {
    const rect = state.floatingIntentBar.getBoundingClientRect();
    dragState = {
      pointerId: event.pointerId,
      offsetX: event.clientX - rect.left,
      offsetY: event.clientY - rect.top
    };

    state.dragHandle.setPointerCapture(event.pointerId);
    state.floatingIntentBar.classList.add("is-dragging");
    setFloatingBarPosition(rect.left, rect.top);
  });

  state.dragHandle.addEventListener("pointermove", (event) => {
    if (!dragState || event.pointerId !== dragState.pointerId) return;
    setFloatingBarPosition(event.clientX - dragState.offsetX, event.clientY - dragState.offsetY);
  });

  state.dragHandle.addEventListener("pointerup", (event) => {
    if (!dragState || event.pointerId !== dragState.pointerId) return;
    dragState = null;
    state.floatingIntentBar.classList.remove("is-dragging");
  });

  state.dragHandle.addEventListener("pointercancel", () => {
    dragState = null;
    state.floatingIntentBar.classList.remove("is-dragging");
  });

  window.addEventListener("resize", () => {
    const rect = state.floatingIntentBar.getBoundingClientRect();
    setFloatingBarPosition(rect.left, rect.top);
  });
}

restoreFloatingBarPosition();
enableFloatingDrag();

openScenario("chat", {
  intent: "告诉张三，今晚吃瓦香鸡",
  contact: "张三",
  message: "今晚吃瓦香鸡。"
});
