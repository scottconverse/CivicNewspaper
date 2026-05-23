// assistant-skill/client.js
const fs = require('fs');
const path = require('path');
const http = require('http');

const TOKEN_PATH = path.join(
  process.env.APPDATA || (process.platform === 'darwin' ? process.env.HOME + '/Library/Application Support' : process.env.HOME),
  'civicnews-token.json'
);

const PORT = 12053;
const HOST = '127.0.0.1';

function loadToken() {
  try {
    if (fs.existsSync(TOKEN_PATH)) {
      const data = JSON.parse(fs.readFileSync(TOKEN_PATH, 'utf8'));
      return data.token;
    }
  } catch (e) {
    console.error('Error loading token:', e.message);
  }
  return null;
}

function saveToken(token) {
  try {
    const dir = path.dirname(TOKEN_PATH);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(TOKEN_PATH, JSON.stringify({ token }, null, 2), 'utf8');
    console.log(`Pairing token saved to ${TOKEN_PATH}`);
  } catch (e) {
    console.error('Failed to save pairing token:', e.message);
  }
}

function makeRequest(method, urlPath, body = null) {
  return new Promise((resolve, reject) => {
    const token = loadToken();
    const headers = {
      'Host': `${HOST}:${PORT}`,
      'Content-Type': 'application/json'
    };

    if (token && urlPath !== '/api/pair') {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const options = {
      hostname: HOST,
      port: PORT,
      path: urlPath,
      method: method,
      headers: headers
    };

    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => { data += chunk; });
      res.on('end', () => {
        if (res.statusCode >= 200 && res.statusCode < 300) {
          try {
            resolve(JSON.parse(data));
          } catch (e) {
            resolve(data);
          }
        } else {
          reject(new Error(`HTTP ${res.statusCode}: ${data}`));
        }
      });
    });

    req.on('error', (err) => {
      reject(new Error(`Connection failed: ${err.message}. Is CivicNews Tauri app running?`));
    });

    if (body) {
      req.write(JSON.stringify(body));
    }
    req.end();
  });
}

async function main() {
  const args = process.argv.slice(2);
  const command = args[0];

  if (!command) {
    console.log(`
CivicNews Coding Assistant CLI Bridge
Usage:
  node client.js pair <pin>                 - Pair with active desktop PIN code
  node client.js queue                      - Fetch today's story queue
  node client.js evidence <lead_id>          - Fetch evidence items for a lead
  node client.js draft <lead_id> <format> <title> <content> - Submit a story draft
  node client.js check <draft_id>            - Run pre-publish guardrails
  node client.js llm <prompt> [system]      - Run a rate-limited local LLM task
    `);
    process.exit(0);
  }

  try {
    switch (command) {
      case 'pair': {
        const pin = args[1];
        if (!pin) {
          console.error('Error: Please provide pairing token.');
          process.exit(1);
        }
        console.log(`Attempting pairing PIN ${pin}...`);
        const res = await makeRequest('POST', '/api/pair', { pin });
        if (res.token) {
          saveToken(res.token);
          console.log('Pairing successful!');
        } else {
          console.error('Failed: No token returned.');
        }
        break;
      }

      case 'queue': {
        const res = await makeRequest('GET', '/api/queue');
        console.log(JSON.stringify(res, null, 2));
        break;
      }

      case 'evidence': {
        const leadId = parseInt(args[1], 10);
        if (isNaN(leadId)) {
          console.error('Error: Please provide a valid lead ID.');
          process.exit(1);
        }
        const res = await makeRequest('GET', `/api/evidence/${leadId}`);
        console.log(JSON.stringify(res, null, 2));
        break;
      }

      case 'draft': {
        const leadId = args[1] === 'null' ? null : parseInt(args[1], 10);
        const format = args[2];
        const title = args[3];
        const content = args[4];

        if (!format || !title || !content) {
          console.error('Usage: node client.js draft <lead_id|null> <format> <title> <content>');
          process.exit(1);
        }

        const res = await makeRequest('POST', '/api/drafts', {
          lead_id: leadId,
          format,
          title,
          content
        });
        console.log(JSON.stringify(res, null, 2));
        break;
      }

      case 'check': {
        const draftId = parseInt(args[1], 10);
        if (isNaN(draftId)) {
          console.error('Error: Please provide a valid draft ID.');
          process.exit(1);
        }
        const res = await makeRequest('POST', '/api/guardrails/check', { draft_id: draftId });
        console.log(JSON.stringify(res, null, 2));
        break;
      }

      case 'llm': {
        const prompt = args[1];
        const system = args[2] || 'You are a helpful assistant.';
        if (!prompt) {
          console.error('Error: Please provide a prompt.');
          process.exit(1);
        }
        const res = await makeRequest('POST', '/api/llm/task', { prompt, system });
        console.log(JSON.stringify(res, null, 2));
        break;
      }

      default:
        console.error(`Unknown command: ${command}`);
        process.exit(1);
    }
  } catch (e) {
    console.error('Error:', e.message);
    process.exit(1);
  }
}

main();
