// browser-extension/chromium/background.js
const API_URL = 'http://127.0.0.1:12053';

async function readJsonIfPossible(res) {
  const text = await res.text();
  if (!text) return null;
  try {
    return JSON.parse(text);
  } catch (_err) {
    return { message: text };
  }
}

function statusMessage(status, fallback = 'The desktop app rejected this request.') {
  if (status === 401) return 'Pairing expired or unauthorized. Pair the extension again.';
  if (status === 403) return 'The desktop app rejected this request. Check browser pairing and allowed origin.';
  if (status === 429) return 'Too many attempts. Wait a minute and try again.';
  return `${fallback} HTTP ${status}.`;
}

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.type === 'CN_PAIR') {
    handlePair(message.pin, sendResponse);
    return true; // Keep message channel open for async response
  } else if (message.type === 'CN_GET_QUEUE') {
    handleGetQueue(sendResponse);
    return true;
  } else if (message.type === 'CN_GET_EVIDENCE') {
    handleGetEvidence(message.leadId, sendResponse);
    return true;
  } else if (message.type === 'CN_SUBMIT_DRAFT') {
    handleSubmitDraft(message.draft, sendResponse);
    return true;
  }
});

function handlePair(pin, sendResponse) {
  fetch(`${API_URL}/api/pair`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-civicnews-pair': '1'
    },
    body: JSON.stringify({ pin })
  })
    .then(async (res) => {
      const data = await readJsonIfPossible(res);
      if (res.ok && data && data.token) {
        chrome.storage.local.set({ civicnews_token: data.token }, () => {
          sendResponse({ success: true, token: data.token });
        });
      } else {
        sendResponse({ success: false, error: res.status === 429 ? statusMessage(res.status) : 'Invalid code or pairing expired.' });
      }
    })
    .catch((err) => {
      sendResponse({ success: false, error: `Connection failed: ${err.message}` });
    });
}

function handleGetQueue(sendResponse) {
  chrome.storage.local.get(['civicnews_token'], (result) => {
    const token = result.civicnews_token;
    if (!token) {
      sendResponse({ success: false, error: 'Not paired. Click the Civic Desk extension icon to pair.' });
      return;
    }

    fetch(`${API_URL}/api/queue`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })
      .then(async (res) => {
        const data = await readJsonIfPossible(res);
        if (res.ok) {
          sendResponse({ success: true, data });
        } else {
          sendResponse({ success: false, error: statusMessage(res.status, 'Could not fetch the story queue.') });
        }
      })
      .catch((err) => {
        sendResponse({ success: false, error: `Inference failed: ${err.message}` });
      });
  });
}

function handleGetEvidence(leadId, sendResponse) {
  chrome.storage.local.get(['civicnews_token'], (result) => {
    const token = result.civicnews_token;
    if (!token) {
      sendResponse({ success: false, error: 'Not paired.' });
      return;
    }

    fetch(`${API_URL}/api/evidence/${leadId}`, {
      headers: {
        'Authorization': `Bearer ${token}`
      }
    })
      .then(async (res) => {
        const data = await readJsonIfPossible(res);
        if (res.ok) {
          sendResponse({ success: true, data });
        } else {
          sendResponse({ success: false, error: statusMessage(res.status, 'Could not fetch evidence.') });
        }
      })
      .catch((err) => {
        sendResponse({ success: false, error: err.message });
      });
  });
}

function handleSubmitDraft(draft, sendResponse) {
  chrome.storage.local.get(['civicnews_token'], (result) => {
    const token = result.civicnews_token;
    if (!token) {
      sendResponse({ success: false, error: 'Not paired.' });
      return;
    }

    fetch(`${API_URL}/api/drafts`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(draft)
    })
      .then(async (res) => {
        const data = await readJsonIfPossible(res);
        if (res.ok) {
          sendResponse({ success: true, data });
        } else {
          sendResponse({ success: false, error: statusMessage(res.status, 'Could not submit the draft.') });
        }
      })
      .catch((err) => {
        sendResponse({ success: false, error: err.message });
      });
  });
}
