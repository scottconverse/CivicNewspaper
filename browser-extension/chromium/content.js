// browser-extension/chromium/content.js

// 1. Inject Styles
const style = document.createElement('style');
style.textContent = `
  #civicnews-bridge-root {
    position: fixed;
    bottom: 20px;
    right: 20px;
    z-index: 99999;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  }
  .cn-float-btn {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background-color: #0f172a;
    color: #38bdf8;
    border: 2px solid #334155;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }
  .cn-float-btn:hover {
    transform: scale(1.05);
    background-color: #1e293b;
    border-color: #38bdf8;
  }
  .cn-float-btn svg {
    width: 24px;
    height: 24px;
    fill: none;
    stroke: currentColor;
    stroke-width: 2;
  }
  .cn-drawer {
    display: none;
    position: absolute;
    bottom: 60px;
    right: 0;
    width: 360px;
    max-height: 500px;
    background: rgba(15, 23, 42, 0.95);
    backdrop-filter: blur(8px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    color: #f8fafc;
    flex-direction: column;
    overflow: hidden;
  }
  .cn-drawer.open {
    display: flex;
  }
  .cn-header {
    padding: 12px 16px;
    background: #090d16;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .cn-title {
    font-weight: 600;
    font-size: 14px;
    color: #38bdf8;
  }
  .cn-close {
    background: none;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    font-size: 18px;
  }
  .cn-body {
    padding: 16px;
    overflow-y: auto;
    flex-grow: 1;
    font-size: 13px;
  }
  .cn-pair-box {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .cn-input {
    padding: 8px 12px;
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 6px;
    color: white;
    outline: none;
  }
  .cn-button {
    padding: 8px 12px;
    background: #38bdf8;
    color: #0f172a;
    border: none;
    border-radius: 6px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  .cn-button:hover {
    background: #7dd3fc;
  }
  .cn-button-sec {
    padding: 6px 10px;
    background: transparent;
    border: 1px solid #334155;
    color: #94a3b8;
    border-radius: 6px;
    cursor: pointer;
  }
  .cn-button-sec:hover {
    color: white;
    background: rgba(255,255,255,0.05);
  }
  .cn-lead-item {
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    padding: 8px;
    margin-bottom: 8px;
    background: rgba(255, 255, 255, 0.02);
    cursor: pointer;
    transition: background 0.15s;
  }
  .cn-lead-item:hover {
    background: rgba(255, 255, 255, 0.06);
  }
  .cn-badge-red {
    background: rgba(239, 68, 68, 0.2);
    color: #f87171;
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 4px;
  }
  .cn-badge-blue {
    background: rgba(56, 189, 248, 0.2);
    color: #38bdf8;
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 4px;
  }
  .cn-lead-why {
    font-weight: 500;
    margin-top: 4px;
    line-height: 1.4;
  }
  .cn-log {
    margin-top: 8px;
    padding: 6px;
    background: #020617;
    border-radius: 4px;
    font-family: monospace;
    font-size: 11px;
    color: #10b981;
    max-height: 80px;
    overflow-y: auto;
  }
`;
document.head.appendChild(style);

// 2. Build DOM structures
const root = document.createElement('div');
root.id = 'civicnews-bridge-root';

const floatBtn = document.createElement('button');
floatBtn.className = 'cn-float-btn';
floatBtn.innerHTML = `
  <svg viewBox="0 0 24 24">
    <path d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10l4 4v10a2 2 0 01-2 2z"></path>
    <path d="M14 4v4h4"></path>
    <path d="M7 8h4"></path>
    <path d="M7 12h10"></path>
    <path d="M7 16h10"></path>
  </svg>
`;

const drawer = document.createElement('div');
drawer.className = 'cn-drawer';

root.appendChild(floatBtn);
root.appendChild(drawer);
document.body.appendChild(root);

let state = {
  paired: false,
  leads: [],
  logs: ''
};

// Toggle drawer
floatBtn.addEventListener('click', () => {
  drawer.classList.toggle('open');
  if (drawer.classList.contains('open')) {
    checkPairingAndLoad();
  }
});

function appendLog(msg) {
  state.logs = msg;
  render();
}

// Check storage and load data
function checkPairingAndLoad() {
  chrome.storage.local.get(['civicnews_token'], (res) => {
    if (res.civicnews_token) {
      state.paired = true;
      fetchQueue();
    } else {
      state.paired = false;
      render();
    }
  });
}

function fetchQueue() {
  appendLog('Loading queue...');
  chrome.runtime.sendMessage({ type: 'CN_GET_QUEUE' }, (response) => {
    if (response && response.success) {
      state.leads = response.data.leads || [];
      state.paired = true;
      appendLog('');
    } else {
      state.leads = [];
      state.paired = false;
      appendLog(response ? response.error : 'Connection lost to CivicNews core API.');
    }
    render();
  });
}

function handlePairSubmit(pin) {
  appendLog('Pairing with PIN...');
  chrome.runtime.sendMessage({ type: 'CN_PAIR', pin }, (response) => {
    if (response && response.success) {
      state.paired = true;
      appendLog('Paired successfully!');
      fetchQueue();
    } else {
      state.paired = false;
      appendLog(response ? response.error : 'Pairing failed.');
    }
    render();
  });
}

function insertTextIntoChat(text) {
  const selectors = [
    '#prompt-textarea',
    '[contenteditable="true"]',
    'textarea',
    '#chat-input'
  ];

  let textarea = null;
  for (const selector of selectors) {
    const el = document.querySelector(selector);
    if (el) {
      textarea = el;
      break;
    }
  }

  if (!textarea) {
    alert('Could not find prompt textbox on this page. Focus on the chat window first!');
    return false;
  }

  if (textarea.tagName !== 'TEXTAREA' && textarea.getAttribute('contenteditable') === 'true') {
    textarea.focus();
    // Dispatch input after setting textContent
    textarea.textContent = text;
    textarea.dispatchEvent(new Event('input', { bubbles: true }));
  } else {
    textarea.focus();
    textarea.value = text;
    textarea.dispatchEvent(new Event('input', { bubbles: true }));
  }
  return true;
}

// Request evidence and insert prompt
function loadLeadEvidenceAndInsert(lead) {
  appendLog(`Fetching evidence for lead #${lead.id}...`);
  chrome.runtime.sendMessage({ type: 'CN_GET_EVIDENCE', leadId: lead.id }, (response) => {
    if (response && response.success) {
      const items = response.data || [];
      let promptText = `I am drafting a story on the following local news lead:\n"${lead.why}"\n\n`;
      promptText += `Here are the raw public records evidence excerpts:\n`;
      
      items.forEach((item) => {
        promptText += `\n--- Evidence ID: ${item.id} ---\nExcerpt: ${item.excerpt}\n`;
      });
      
      promptText += `\nInstructions:\nPlease draft a balanced, objective, third-person report based strictly on the provided evidence. Cite facts with standard Markdown links referring to their evidence IDs, like [Source](evidence:ID). Do not use sensationalized terms, and maintain a presumption of innocence.`;
      
      const success = insertTextIntoChat(promptText);
      if (success) {
        appendLog('Evidence packet inserted into chat prompt box!');
      }
    } else {
      appendLog(`Failed to fetch evidence: ${response ? response.error : 'Unknown'}`);
    }
    render();
  });
}

// Scan DOM for latest assistant response and submit as draft
function captureAssistantResponseAndSubmit(leadId) {
  // Try to find the last assistant message box.
  // claude.ai: font-clarity messages, chatgpt: .agent-turn / .markdown
  const possibleSelectors = [
    'div[data-testid="message"]',
    '.font-clarity',
    '.message',
    '.agent-turn',
    '.markdown'
  ];
  
  let messages = [];
  for (const selector of possibleSelectors) {
    const els = document.querySelectorAll(selector);
    if (els.length > 0) {
      messages = Array.from(els);
      break;
    }
  }

  if (messages.length === 0) {
    alert('Could not find any assistant messages to capture in DOM.');
    return;
  }

  // Get last message text
  const lastMsgEl = messages[messages.length - 1];
  const content = lastMsgEl.innerText || lastMsgEl.textContent;
  
  if (!content || content.length < 50) {
    alert('Captured text is too short or empty.');
    return;
  }

  const title = prompt('Enter a title for this draft:', 'Draft: Captured from Browser Assistant');
  if (!title) return;

  appendLog('Submitting draft to CivicNews workbench...');
  chrome.runtime.sendMessage({
    type: 'CN_SUBMIT_DRAFT',
    draft: {
      lead_id: leadId,
      format: 'watch',
      title: title,
      content: content
    }
  }, (response) => {
    if (response && response.success) {
      appendLog('Draft submitted successfully! View in Story Workbench.');
    } else {
      appendLog(`Failed to submit draft: ${response ? response.error : 'Unknown'}`);
    }
    render();
  });
}

// 3. Render UI based on state
function render() {
  drawer.innerHTML = '';

  // Header
  const header = document.createElement('div');
  header.className = 'cn-header';
  header.innerHTML = `
    <span class="cn-title">CivicNews Bridge</span>
    <button class="cn-close" aria-label="Close">x</button>
  `;
  header.querySelector('.cn-close').addEventListener('click', () => {
    drawer.classList.remove('open');
  });
  drawer.appendChild(header);

  const body = document.createElement('div');
  body.className = 'cn-body';

  if (!state.paired) {
    // Render pairing form
    body.innerHTML = `
      <div class="cn-pair-box">
        <h4 style="margin:0 0 5px 0;">Pair local bridge</h4>
        <p style="color:#94a3b8;margin:0 0 10px 0;line-height:1.4;">Open The Civic Desk &gt; Browser Pairing, generate a code, then paste it here.</p>
        <input type="text" id="cn-pin-input" class="cn-input" placeholder="Paste pairing code" />
        <button id="cn-pair-btn" class="cn-button">Pair extension</button>
      </div>
    `;
    body.querySelector('#cn-pair-btn').addEventListener('click', () => {
      const pin = body.querySelector('#cn-pin-input').value;
      if (pin) handlePairSubmit(pin);
    });
  } else {
    // Render leads queue
    const listTitle = document.createElement('h4');
    listTitle.style.margin = '0 0 10px 0';
    listTitle.textContent = `Story Leads (${state.leads.length})`;
    body.appendChild(listTitle);

    if (state.leads.length === 0) {
      const empty = document.createElement('p');
      empty.style.color = '#94a3b8';
      empty.textContent = 'No leads today. Sync or scrape inside the main app.';
      body.appendChild(empty);
    } else {
      const container = document.createElement('div');
      state.leads.forEach((lead) => {
        const item = document.createElement('div');
        item.className = 'cn-lead-item';
        const topRow = document.createElement('div');
        topRow.style.cssText = "display:flex;justify-content:space-between;";
        
        const riskBadge = document.createElement('span');
        riskBadge.className = lead.risk_level === 'high' ? 'cn-badge-red' : 'cn-badge-blue';
        riskBadge.textContent = lead.risk_level.toUpperCase();
        
        const idSpan = document.createElement('span');
        idSpan.style.cssText = "color:#64748b;font-size:10px;";
        idSpan.textContent = `ID: ${lead.id}`;
        
        topRow.appendChild(riskBadge);
        topRow.appendChild(idSpan);
        
        const whyDiv = document.createElement('div');
        whyDiv.className = 'cn-lead-why';
        whyDiv.textContent = lead.why;
        
        const actionRow = document.createElement('div');
        actionRow.style.cssText = "display:flex;gap:5px;margin-top:8px;justify-content:flex-end;";
        
        const injectBtn = document.createElement('button');
        injectBtn.className = 'cn-button-sec cn-inject-btn';
        injectBtn.style.cssText = "font-size:11px;";
        injectBtn.textContent = 'Insert Prompt';
        
        const captureBtn = document.createElement('button');
        captureBtn.className = 'cn-button-sec cn-capture-btn';
        captureBtn.style.cssText = "font-size:11px;";
        captureBtn.textContent = 'Capture Draft';
        
        actionRow.appendChild(injectBtn);
        actionRow.appendChild(captureBtn);
        
        item.appendChild(topRow);
        item.appendChild(whyDiv);
        item.appendChild(actionRow);
        
        item.querySelector('.cn-inject-btn').addEventListener('click', (e) => {
          e.stopPropagation();
          loadLeadEvidenceAndInsert(lead);
        });

        item.querySelector('.cn-capture-btn').addEventListener('click', (e) => {
          e.stopPropagation();
          captureAssistantResponseAndSubmit(lead.id);
        });

        container.appendChild(item);
      });
      body.appendChild(container);
    }
  }

  // Logs / Status feedback
  if (state.logs) {
    const logBox = document.createElement('div');
    logBox.className = 'cn-log';
    logBox.textContent = state.logs;
    body.appendChild(logBox);
  }

  drawer.appendChild(body);
}

render();
console.log('CivicNews Browser Bridge active.');
