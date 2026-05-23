// browser-extension/safari/background.js
const API_URL = 'http://127.0.0.1:12053';

// Use browser API namespace if available, fallback to chrome
const ext = typeof browser !== 'undefined' ? browser : chrome;

ext.runtime.onMessage.addListener((message, sender, sendResponse) => {
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
      'Host': '127.0.0.1:12053'
    },
    body: JSON.stringify({ pin })
  })
    .then(async (res) => {
      const data = await res.json();
      if (res.ok && data.token) {
        ext.storage.local.set({ civicnews_token: data.token }, () => {
          sendResponse({ success: true, token: data.token });
        });
      } else {
        sendResponse({ success: false, error: 'Invalid PIN or pairing expired.' });
      }
    })
    .catch((err) => {
      sendResponse({ success: false, error: `Connection failed: ${err.message}` });
    });
}

function handleGetQueue(sendResponse) {
  ext.storage.local.get(['civicnews_token'], (result) => {
    const token = result.civicnews_token;
    if (!token) {
      sendResponse({ success: false, error: 'Not paired. Click the CivicNews icon to pair.' });
      return;
    }

    fetch(`${API_URL}/api/queue`, {
      headers: {
        'Authorization': `Bearer ${token}`,
        'Host': '127.0.0.1:12053'
      }
    })
      .then(async (res) => {
        const data = await res.json();
        if (res.ok) {
          sendResponse({ success: true, data });
        } else {
          sendResponse({ success: false, error: `Server returned HTTP ${res.status}` });
        }
      })
      .catch((err) => {
        sendResponse({ success: false, error: `Inference failed: ${err.message}` });
      });
  });
}

function handleGetEvidence(leadId, sendResponse) {
  ext.storage.local.get(['civicnews_token'], (result) => {
    const token = result.civicnews_token;
    if (!token) {
      sendResponse({ success: false, error: 'Not paired.' });
      return;
    }

    fetch(`${API_URL}/api/evidence/${leadId}`, {
      headers: {
        'Authorization': `Bearer ${token}`,
        'Host': '127.0.0.1:12053'
      }
    })
      .then(async (res) => {
        const data = await res.json();
        if (res.ok) {
          sendResponse({ success: true, data });
        } else {
          sendResponse({ success: false, error: `Server returned HTTP ${res.status}` });
        }
      })
      .catch((err) => {
        sendResponse({ success: false, error: err.message });
      });
  });
}

function handleSubmitDraft(draft, sendResponse) {
  ext.storage.local.get(['civicnews_token'], (result) => {
    const token = result.civicnews_token;
    if (!token) {
      sendResponse({ success: false, error: 'Not paired.' });
      return;
    }

    fetch(`${API_URL}/api/drafts`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
        'Host': '127.0.0.1:12053'
      },
      body: JSON.stringify(draft)
    })
      .then(async (res) => {
        const data = await res.json();
        if (res.ok) {
          sendResponse({ success: true, data });
        } else {
          sendResponse({ success: false, error: `Server returned HTTP ${res.status}` });
        }
      })
      .catch((err) => {
        sendResponse({ success: false, error: err.message });
      });
  });
}
