const fs = require('node:fs/promises');
require('module').Module._initPaths();
const { chromium } = require('playwright');
async function cap(page,out,name){
  await page.screenshot({path:`${out}/${name}.png`, fullPage:true});
  const text=await page.locator('body').innerText().catch(async()=>await page.textContent('body')) || '';
  await fs.writeFile(`${out}/${name}.txt`, text, 'utf8');
  const info=await page.evaluate(()=>({buttons:[...document.querySelectorAll('button')].map((b,i)=>({i,text:b.innerText,aria:b.getAttribute('aria-label'),disabled:b.disabled})), inputs:[...document.querySelectorAll('input,textarea,select')].map((el,i)=>({i,tag:el.tagName,type:el.getAttribute('type'),value:el.value,placeholder:el.getAttribute('placeholder'),label:el.getAttribute('aria-label'),name:el.getAttribute('name')}))}));
  await fs.writeFile(`${out}/${name}.json`, JSON.stringify(info,null,2),'utf8');
  return text;
}
(async()=>{
  const out=process.argv[2];
  const browser=await chromium.connectOverCDP('http://127.0.0.1:9224');
  const page=browser.contexts()[0].pages()[0];
  let states=[];
  for(let i=0;i<8;i++){
    await page.waitForTimeout(3000);
    const text=await cap(page,out,`cdp-11-wait-starter-${i}`);
    states.push({i, hasSetupTask:/Setup task/i.test(text), hasAdding:/Adding starter sources/i.test(text), top:text.split('\n').slice(0,18)});
    if(!/Adding starter sources/i.test(text)) break;
  }
  await fs.writeFile(`${out}/starter-source-wait-states.json`, JSON.stringify(states,null,2),'utf8');
  const daily = page.getByText('Daily Scan', {exact:true}).first();
  await daily.click();
  await page.waitForTimeout(1200);
  await cap(page,out,'cdp-12-daily-before-run');
  await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
