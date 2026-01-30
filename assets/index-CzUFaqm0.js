(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const n of document.querySelectorAll('link[rel="modulepreload"]'))o(n);new MutationObserver(n=>{for(const s of n)if(s.type==="childList")for(const a of s.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&o(a)}).observe(document,{childList:!0,subtree:!0});function r(n){const s={};return n.integrity&&(s.integrity=n.integrity),n.referrerPolicy&&(s.referrerPolicy=n.referrerPolicy),n.crossOrigin==="use-credentials"?s.credentials="include":n.crossOrigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function o(n){if(n.ep)return;n.ep=!0;const s=r(n);fetch(n.href,s)}})();const m=Object.freeze({HACKERNEWS:"hackernews",GITHUB:"github",REDDIT:"reddit",PRODUCTHUNT:"producthunt",ARXIV:"arxiv"}),I=new Set(Object.values(m));function v({id:e,title:t,url:r,source:o,score:n,comments:s,author:a,timestamp:c,tags:i,discussionUrl:l,summary:u}){if(typeof e!="string"||e.length===0)throw new Error(`Story: id must be a non-empty string, got "${e}"`);if(typeof t!="string"||t.length===0)throw new Error(`Story: title must be a non-empty string, got "${t}"`);if(!I.has(o))throw new Error(`Story: source must be one of [${[...I].join(", ")}], got "${o}"`);return Object.freeze({id:e,title:t,url:typeof r=="string"?r:"",source:o,score:O(n,0,100),comments:O(s,0,1/0),author:typeof a=="string"&&a.length>0?a:"unknown",timestamp:typeof c=="number"?c:Math.floor(Date.now()/1e3),tags:Object.freeze(Array.isArray(i)?[...i]:[]),discussionUrl:typeof l=="string"?l:"",summary:typeof u=="string"?u:""})}function O(e,t,r){return typeof e!="number"||isNaN(e)?t:Math.max(t,Math.min(r,Math.round(e)))}const N=new Set(["the","a","an","is","are","was","were","be","been","being","have","has","had","do","does","did","will","would","could","should","may","might","can","shall","to","of","in","for","on","with","at","by","from","as","into","through","during","before","after","above","below","between","out","off","up","down","about","or","and","but","not","no","nor","so","yet","both","either","neither","each","every","all","any","few","more","most","other","some","such","than","too","very","just","because","if","when","while","how","what","which","who","whom","this","that","these","those","it","its","i","me","my","we","our","you","your","he","him","his","she","her","they","them","their","new","now","get","got","make","made","way","back","show","ask","tell","use","using","used","why","via","vs","like","one","two","first","also","even","still","already","here","there","says","said","lets","let","see","look","need","want","think","know","work","working","really","much","many","well","only","over","year","years","day","days","time","long","part","things","thing","goes","going","come","better","best","big","small","old","next","open","source","free","built","build","building","people","world","today","never","keep","take"]),z=Object.freeze({gpt4:"gpt-4","gpt-4o":"gpt-4",gpt4o:"gpt-4",gpt5:"gpt-5",llms:"llm",genai:"generative-ai","gen-ai":"generative-ai",js:"javascript",ts:"typescript",reactjs:"react","react.js":"react",vuejs:"vue","vue.js":"vue",nodejs:"node","node.js":"node",nextjs:"next.js","next.js":"next.js",golang:"go",rustlang:"rust",py:"python",cpp:"c++",gh:"github",k8s:"kubernetes",tf:"terraform",postgres:"postgresql"});function b(e){if(typeof e!="string"||e.length===0)return[];const r=e.toLowerCase().replace(/[^a-z0-9\s\-\.]/g," ").replace(/\s+/g," ").trim().split(" "),o=[];for(const n of r){if(n.length<=1||N.has(n))continue;const s=n.replace(/^[\-\.]+|[\-\.]+$/g,"");if(s.length<=1)continue;const a=z[s]||s;o.push(a)}return[...new Set(o)]}const _="https://hacker-news.firebaseio.com/v0";async function B({limit:e=30}={}){try{const t=await fetch(`${_}/topstories.json`);if(!t.ok)throw new Error(`HN API error: ${t.status}`);const r=await t.json();return(await Promise.all(r.slice(0,e).map(n=>fetch(`${_}/item/${n}.json`).then(s=>s.json()).catch(()=>null)))).filter(n=>n&&n.title).map(n=>v({id:`hn:${n.id}`,title:n.title,url:n.url||"",source:m.HACKERNEWS,score:F(n.score||0),comments:n.descendants||0,author:n.by||"unknown",timestamp:n.time||Math.floor(Date.now()/1e3),tags:b(n.title),discussionUrl:`https://news.ycombinator.com/item?id=${n.id}`}))}catch(t){return console.error("[adapter:hn] Failed to fetch stories:",t),[]}}function F(e){return e<=0?0:Math.min(100,Math.round(Math.log2(e+1)/15*100))}const V=Object.freeze(Object.defineProperty({__proto__:null,fetchStories:B},Symbol.toStringTag,{value:"Module"})),G="https://api.github.com/search/repositories";async function K({limit:e=30}={}){try{const t=new Date;t.setDate(t.getDate()-7);const o=`created:>${t.toISOString().split("T")[0]}`,n=`${G}?q=${encodeURIComponent(o)}&sort=stars&order=desc&per_page=${e}`,s=await fetch(n,{headers:{Accept:"application/vnd.github.v3+json"}});if(!s.ok){if(s.status===403||s.status===429)return console.warn("[github] Rate limit exceeded, returning empty list"),[];throw new Error(`GitHub API error: ${s.status}`)}return((await s.json()).items||[]).map(Y)}catch(t){return console.error("[github] Fetch failed:",t),[]}}function Y(e){return v({id:`gh-${e.id}`,title:`${e.full_name}: ${e.description||"No description"}`,url:e.html_url,source:m.GITHUB,score:W(e.stargazers_count),comments:e.forks_count,author:e.owner?e.owner.login:"unknown",timestamp:new Date(e.created_at).getTime()/1e3,tags:b(`${e.name} ${e.description||""} ${e.language||""}`),discussionUrl:`${e.html_url}/issues`,summary:e.description||"No description available."})}function W(e){return e>1e3?100:Math.min(100,Math.round(e/1e3*100))}const X=Object.freeze(Object.defineProperty({__proto__:null,fetchStories:K},Symbol.toStringTag,{value:"Module"})),M=["programming","technology","machinelearning","javascript","webdev"];async function Q({limit:e=30}={}){try{const t=M.map(n=>J(n,Math.ceil(e/M.length))),r=await Promise.allSettled(t),o=[];for(const n of r)n.status==="fulfilled"&&o.push(...n.value);return o}catch(t){return console.error("[reddit] Fetch failed:",t),[]}}async function J(e,t){try{const r="https://corsproxy.io/?",o=encodeURIComponent(`https://www.reddit.com/r/${e}/hot.json?limit=${t}`),n=await fetch(`${r}${o}`);if(!n.ok)throw new Error(`Reddit API error: ${n.status}`);const s=await n.json();return!s.data||!s.data.children?[]:s.data.children.filter(a=>!a.data.stickied).map(a=>Z(a.data))}catch(r){return console.warn(`[reddit] Failed to fetch r/${e}:`,r),[]}}function Z(e){return v({id:`reddit-${e.id}`,title:e.title,url:e.url,source:m.REDDIT,score:ee(e.score),comments:e.num_comments,author:e.author,timestamp:e.created_utc,tags:b(`${e.title} ${e.subreddit}`),discussionUrl:`https://reddit.com${e.permalink}`,summary:e.selftext?e.selftext.substring(0,300)+(e.selftext.length>300?"...":""):""})}function ee(e){return e>500?100:Math.min(100,Math.round(e/500*100))}const te=Object.freeze(Object.defineProperty({__proto__:null,fetchStories:Q},Symbol.toStringTag,{value:"Module"})),re="https://www.producthunt.com/feed",ne=`https://api.rss2json.com/v1/api.json?rss_url=${encodeURIComponent(re)}`;async function oe({limit:e=30}={}){try{const t=await fetch(ne);if(!t.ok)throw new Error(`RSS Bridge error: ${t.status}`);const r=await t.json();if(r.status!=="ok")throw new Error("RSS Bridge returned error status");return(r.items||[]).slice(0,e).map(se)}catch(t){return console.error("[producthunt] Fetch failed:",t),[]}}function se(e){const t=e.guid&&typeof e.guid=="string"?e.guid:"",r=typeof e.title=="string"?e.title:"Untitled Product",o=t.split("/").pop()||r.replace(/\s+/g,"-");return v({id:`ph-${o}`,title:r,url:e.link,source:m.PRODUCTHUNT,score:80,comments:0,author:e.author||"Product Hunt",timestamp:new Date(e.pubDate).getTime()/1e3,tags:b(`${e.title} ${e.categories?e.categories.join(" "):""}`),discussionUrl:e.link,summary:e.description||""})}const ae=Object.freeze(Object.defineProperty({__proto__:null,fetchStories:oe},Symbol.toStringTag,{value:"Module"})),ie="http://export.arxiv.org/api/query";async function ce({limit:e=30}={}){try{const t="cat:cs.AI OR cat:cs.SE OR cat:cs.LG OR cat:cs.CV",r="https://corsproxy.io/?",o=encodeURIComponent(`${ie}?search_query=${encodeURIComponent(t)}&start=0&max_results=${e}&sortBy=submittedDate&sortOrder=descending`),n=await fetch(`${r}${o}`);if(!n.ok)throw new Error(`ArXiv API error: ${n.status}`);const s=await n.text(),c=new DOMParser().parseFromString(s,"text/xml");return Array.from(c.querySelectorAll("entry")).map(le)}catch(t){return console.error("[arxiv] Fetch failed:",t),[]}}function le(e){const t=e.querySelector("id").textContent,r=t.split("/").pop(),o=e.querySelector("title").textContent.replace(/\n/g," ").trim(),n=e.querySelector("summary").textContent.replace(/\n/g," ").trim(),s=e.querySelector("published").textContent,a=e.querySelector("author name"),c=a?a.textContent:"ArXiv",i=e.querySelector('link[title="pdf"]')?e.querySelector('link[title="pdf"]').getAttribute("href"):t;return v({id:`arxiv-${r}`,title:`[Paper] ${o}`,url:i,source:m.ARXIV,score:60,comments:0,author:c,timestamp:new Date(s).getTime()/1e3,tags:b(`${o} ${n}`),discussionUrl:t,summary:n})}const de=Object.freeze(Object.defineProperty({__proto__:null,fetchStories:ce},Symbol.toStringTag,{value:"Module"})),ue="techpulse_";function S(e,{version:t,migrations:r={},defaultValue:o}){if(typeof e!="string"||e.length===0)throw new Error("createStore: key must be a non-empty string");if(typeof t!="number"||t<1||!Number.isInteger(t))throw new Error("createStore: version must be a positive integer");if(typeof o!="function")throw new Error("createStore: defaultValue must be a factory function");const n=ue+e;function s(){const i=localStorage.getItem(n);if(i===null)return o();let l;try{l=JSON.parse(i)}catch{return console.warn(`[storage] Corrupted data for "${e}", resetting to default.`),o()}return pe(l,t,r,e)}function a(i){const l={...i,_version:t};localStorage.setItem(n,JSON.stringify(l))}function c(){localStorage.removeItem(n)}return Object.freeze({read:s,write:a,clear:c})}function pe(e,t,r,o){let n=e._version||0;for(;n<t;){n++;const s=r[n];if(s)try{e=s(e)}catch(a){return console.warn(`[storage] Migration of "${o}" to v${n} failed:`,a),e._version=n-1,e}e._version=n}return e}const x=Object.freeze({enabledSources:["hackernews","github","reddit","producthunt","arxiv"],trendingWindowDays:7,trendingSpikeThreshold:2,storiesPerSource:30}),w=S("settings",{version:2,migrations:{2:e=>{const t=["github","reddit","producthunt","arxiv"],r=new Set(e.enabledSources||[]);return t.forEach(o=>r.add(o)),e.enabledSources=[...r],e}},defaultValue:()=>({_version:2,...x})}),P=Object.freeze({get(e){const t=w.read();return e in t?t[e]:x[e]},set(e,t){const r=w.read();r[e]=t,w.write(r)},getAll(){const e=w.read();return{...x,...e}}}),he={hackernews:V,github:X,reddit:te,producthunt:ae,arxiv:de};let g={data:[],timestamp:0,ttl:300*1e3};async function ge({limit:e,forceRefresh:t=!1}={}){const r=Date.now();if(!t&&g.data.length>0&&r-g.timestamp<g.ttl)return[...g.data];const o=P.getAll(),n=o.enabledSources||[],s=e||o.storiesPerSource||30,a=Object.entries(he).filter(([u])=>n.includes(u));if(a.length===0)return console.warn("[aggregator] No enabled adapters found."),[];const c=await Promise.allSettled(a.map(([,u])=>u.fetchStories({limit:s}))),i=[];for(const u of c)u.status==="fulfilled"?i.push(...u.value):console.warn("[aggregator] Adapter failed:",u.reason);const l=me(i);return g={data:l,timestamp:r,ttl:g.ttl},l}function me(e){const t=new Map;for(const r of e){const o=r.url||r.id,n=t.get(o);(!n||n.score<r.score)&&t.set(o,r)}return[...t.values()].sort((r,o)=>o.score-r.score||o.timestamp-r.timestamp)}const fe="keyword_history",T=S(fe,{version:1,defaultValue:()=>({history:{},lastProcessed:0})});function ye(e){if(!e||e.length===0)return;const t=T.read(),r=new Date().toISOString().split("T")[0];t.history[r]||(t.history[r]={},ve(t)),e.forEach(o=>{o.tags&&o.tags.forEach(n=>{const s=n.toLowerCase();t.history[r][s]=(t.history[r][s]||0)+1})}),t.lastProcessed=Date.now(),T.write(t)}function ve(e){const t=Object.keys(e.history).sort();t.length>30&&t.slice(0,t.length-30).forEach(o=>delete e.history[o])}function D(){const t=P.getAll().trendingSpikeThreshold||2,r=3,o=T.read(),n=new Date().toISOString().split("T")[0],s=o.history[n]||{},a=Object.keys(o.history).filter(i=>i!==n).sort().reverse().slice(0,7),c=[];return Object.entries(s).forEach(([i,l])=>{if(l<r)return;let u=0,$=0;a.forEach(E=>{o.history[E]&&o.history[E][i]&&(u+=o.history[E][i],$++)});const H=$>0?u/$:.5,j=l/H;j>=t&&c.push({keyword:i,growth:parseFloat(j.toFixed(1)),count:l})}),c.sort((i,l)=>l.growth-i.growth||l.count-i.count).slice(0,10)}const be="user_knowledge",k=S(be,{version:1,defaultValue:()=>({history:[],tagCounts:{}})});function we(e){const t=k.read();t.history.some(r=>r.storyId===e.id)||(t.history.push({storyId:e.id,timestamp:Date.now(),tags:e.tags}),e.tags.forEach(r=>{const o=r.toLowerCase();t.tagCounts[o]=(t.tagCounts[o]||0)+1}),k.write(t))}function Se(){return k.read().tagCounts}function $e(e,t={}){const r=document.createElement("article");r.className="news-card",e.url?Ee(e.url):e.source;const o=xe(e.timestamp),n=e.url||e.discussionUrl;r.innerHTML=`
    <div class="card-content">
      <div class="card-meta-top">
        <span class="source-badge" data-source="${e.source}">${h(e.source)}</span>
        <span class="score">&#9650; ${e.score}</span>
      </div>
      <h2 class="card-title">
        <a href="${h(n)}" target="_blank" rel="noopener noreferrer" class="story-link">
          ${h(e.title)}
        </a>
      </h2>
      <div class="card-tags">
        ${(e.tags||[]).slice(0,3).map(i=>`<span class="tag-badge">#${h(i)}</span>`).join("")}
      </div>
      <div class="card-meta-bottom">
        <span class="author">by ${h(e.author)}</span>
        <span class="time">${h(o)}</span>
      </div>
    </div>
    <div class="card-actions">
      ${e.discussionUrl?`<a href="${h(e.discussionUrl)}" target="_blank" class="comments-link">
            ${e.comments} comments
          </a>`:`<span class="comments-link">${e.comments} comments</span>`}
      ${e.summary?'<button class="summary-btn" aria-label="Toggle Summary" title="Show Summary">✨ Summary</button>':""}
    </div>
    ${e.summary?`<div class="card-summary" style="display: none;">${h(e.summary)}</div>`:""}
  `;const s=r.querySelector(".story-link");s&&t.onRead&&s.addEventListener("click",()=>{t.onRead(e)});const a=r.querySelector(".summary-btn"),c=r.querySelector(".card-summary");return a&&c&&a.addEventListener("click",i=>{i.stopPropagation();const l=c.style.display==="none";c.style.display=l?"block":"none",a.classList.toggle("active",l)}),r}function h(e){if(typeof e!="string")return"";const t=document.createElement("span");return t.textContent=e,t.innerHTML}function Ee(e){try{return new URL(e).hostname.replace("www.","")}catch{return"unknown"}}function xe(e){const t=Math.floor(Date.now()/1e3-e),r=[["year",31536e3],["month",2592e3],["week",604800],["day",86400],["hour",3600],["minute",60]];for(const[o,n]of r){const s=Math.floor(t/n);if(s>=1)return`${s} ${o}${s>1?"s":""} ago`}return"just now"}function Te({onSelect:e}){const t=document.createElement("div");return t.className="trending-bar",ke(t,e),t}function ke(e,t){const r=D();if(r.length===0){e.innerHTML="",e.style.display="none";return}e.style.display="block";const o=document.createElement("div");o.className="trending-label",o.innerHTML="<span>🔥 Trending Now</span>";const n=document.createElement("div");n.className="trending-content",r.forEach(s=>{const a=document.createElement("button");a.className="trending-chip",a.title=`${s.count} stories today`,a.dataset.keyword=s.keyword;const c=s.growth>3?"⚡":"";a.innerHTML=`
            ${c} #${s.keyword}
            <span class="trending-pct">+${Math.round(s.growth*100)}%</span>
        `,a.addEventListener("click",()=>{document.querySelectorAll(".trending-chip").forEach(i=>i.classList.remove("active")),a.classList.add("active"),t&&t(s.keyword)}),n.appendChild(a)}),e.innerHTML="",e.appendChild(o),e.appendChild(n)}const q={all:[],AI:["ai","gpt","gpt-4","gpt-5","llm","chatgpt","claude","gemini","model","neural","machine-learning","deep-learning","generative-ai","openai","anthropic","deepseek","transformer","training","inference","agent","rag"],Web:["css","html","javascript","typescript","react","vue","svelte","web-development","browser","frontend","backend","node","next.js","api","http","wasm","webassembly"],Hardware:["chip","apple","nvidia","intel","amd","processor","hardware","device","phone","gpu","cpu","arm","risc-v"],Science:["space","physics","energy","quantum","science","math","research","biology","chemistry","climate","nasa"]};let d={stories:[],searchQuery:"",currentCategory:"all",trending:[]},p=null,y=[];const Le={async init(e){p=e,d={stories:[],searchQuery:"",currentCategory:"all",trending:[]},y=[],Ae(),je();try{d.stories=await ge(),ye(d.stories),d.trending=D(),Ie()}catch(t){console.error("[view:feed] Init failed:",t);const r=p.querySelector(".news-grid");r&&(r.innerHTML='<div class="loading-state">Error loading content.</div>')}},destroy(){for(const e of y)e();y=[],d={stories:[],searchQuery:"",currentCategory:"all",trending:[]},p=null}};function Ae(){p.innerHTML=`
    <div class="feed-controls">
      <div class="feed-filters">
        ${Object.keys(q).map(e=>`
          <button class="filter-chip${e==="all"?" active":""}" data-category="${e}">
            ${e==="all"?"All":e==="AI"?"AI &amp; ML":e==="Web"?"Web Dev":e}
          </button>
        `).join("")}
      </div>
    </div>
    <div class="trending-bar-container"></div>
    <div class="news-grid">
      <div class="loading-state">Loading latest tech news...</div>
    </div>
  `}function je(){const e=r=>{const o=r.target.closest(".filter-chip");if(o&&p.contains(o)){p.querySelectorAll(".filter-chip").forEach(s=>s.classList.remove("active")),o.classList.add("active"),d.currentCategory=o.dataset.category,L();return}const n=r.target.closest(".trending-chip");if(n&&p.contains(n)){const s=document.getElementById("search-input");s&&(s.value=n.dataset.keyword,s.dispatchEvent(new Event("input")));return}};p.addEventListener("click",e),y.push(()=>p.removeEventListener("click",e));const t=document.getElementById("search-input");if(t){const r=o=>{d.searchQuery=o.target.value.toLowerCase(),L()};t.addEventListener("input",r),y.push(()=>t.removeEventListener("input",r))}}function Ie(){Oe(),L()}function Oe(){const e=p.querySelector(".trending-bar-container");if(!e||(e.innerHTML="",d.trending.length===0))return;const t=Te({onSelect:r=>{const o=document.getElementById("search-input");o&&(o.value=r,o.dispatchEvent(new Event("input")))}});e.appendChild(t)}function L(){const e=p.querySelector(".news-grid");if(!e)return;const t=_e(d.stories);if(t.length===0){e.innerHTML='<div class="loading-state">No stories found matching your criteria.</div>';return}e.innerHTML="";for(const r of t)e.appendChild($e(r,{onRead:o=>we(o)}))}function _e(e){return e.filter(t=>{const r=!d.searchQuery||t.title.toLowerCase().includes(d.searchQuery)||t.tags.some(n=>n.includes(d.searchQuery));let o=!0;if(d.currentCategory!=="all"){const n=q[d.currentCategory]||[];o=t.tags.some(s=>n.includes(s))}return r&&o})}const Me={async init(e){e.innerHTML=`
      <div class="blind-spots-container" style="padding: 2rem; color: var(--text-secondary); text-align: center;">
        <h2 style="color: var(--text-primary); margin-bottom: 1rem;">Your Tech Blind Spots</h2>
        <div id="radar-chart" class="chart-container">
           <!-- Placeholder for visualization -->
           <div class="stat-card">
              <h3>Reading Profile</h3>
              <p>Start reading stories to build your profile!</p>
           </div>
        </div>
        <div id="blind-spot-list" style="margin-top: 2rem; text-align: left; max-width: 600px; margin-left: auto; margin-right: auto;">
        </div>
      </div>
    `,Re(e)},destroy(){}};function Re(e){const t=Se(),r=Object.values(t).reduce((c,i)=>c+i,0);if(r<5)return;const o=e.querySelector("#blind-spot-list"),n=e.querySelector(".stat-card"),a=["ai","web-development","crypto","mobile","devops","security","hardware"].filter(c=>!t[c]||t[c]<2);n.innerHTML=`
    <h3>Profile Strength</h3>
    <div style="font-size: 3rem; font-weight: 800; color: var(--accent-primary); margin: 1rem 0;">
        ${Math.min(100,r*2)}<span style="font-size: 1rem;"> XP</span>
    </div>
    <p>You have read <strong>${r}</strong> stories.</p>
  `,a.length>0?o.innerHTML=`
        <h3 style="color: var(--accent-secondary); margin-bottom: 1rem;">⚠️ Detected Blind Spots</h3>
        <p>You are missing out on updates in these core areas:</p>
        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; margin-top: 1rem;">
            ${a.map(c=>`
                <span style="
                    border: 1px solid var(--accent-secondary); 
                    color: var(--accent-secondary);
                    padding: 4px 12px;
                    border-radius: 100px;
                    font-size: 0.9rem;
                ">
                    ${c}
                </span>
            `).join("")}
        </div>
      `:o.innerHTML=`
        <h3 style="color: #4ade80;">All Clear!</h3>
        <p>You have a well-rounded reading diet across core tech pillars.</p>
      `}const Ce="milestones",A=S(Ce,{version:1,defaultValue:()=>({items:[{id:"seed-1",title:"GPT-4 Release",date:"2023-03-14",description:"OpenAI released GPT-4.",type:"release",tags:["ai","gpt-4"]},{id:"seed-2",title:"React 19 Announcement",date:"2024-04-25",description:"React team announced roadmap for React 19.",type:"announcement",tags:["web","react"]}]})});function Pe(){return[...A.read().items].sort((t,r)=>new Date(r.date)-new Date(t.date))}function De(e){const t=A.read(),r=crypto.randomUUID();return t.items.push({...e,id:r}),A.write(t),r}const qe={async init(e){e.innerHTML=`
      <div class="timeline-layout" style="max-width: 800px; margin: 0 auto; padding: 2rem;">
        <div class="timeline-header" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;">
            <h2 style="font-family: var(--font-heading);">Tech Timeline</h2>
            <button id="btn-add-milestone" style="
                background: hsl(var(--accent-primary));
                color: white;
                border: none;
                padding: 8px 16px;
                border-radius: 100px;
                cursor: pointer;
                font-weight: 600;
            ">+ Add Event</button>
        </div>

        <div id="add-form-container" style="
            background: var(--glass-bg);
            border: 1px solid var(--glass-border);
            padding: 1.5rem;
            border-radius: 12px;
            margin-bottom: 2rem;
            display: none;
        ">
            <!-- Form injected here -->
        </div>

        <div class="timeline-feed" style="position: relative; border-left: 2px solid var(--glass-border); padding-left: 2rem;">
            <!-- Timeline items -->
        </div>
      </div>
    `,Ue(e),U(e)},destroy(){}};function Ue(e){const t=e.querySelector("#btn-add-milestone"),r=e.querySelector("#add-form-container");t.addEventListener("click",()=>{const o=r.style.display==="block";r.style.display=o?"none":"block",o||He(r,e)})}function He(e,t){e.innerHTML=`
        <form id="milestone-form" style="display: grid; gap: 1rem;">
            <input type="text" name="title" placeholder="Event Title" required style="
                background: rgba(255,255,255,0.05); border: 1px solid var(--glass-border); padding: 10px; color: white; border-radius: 6px;
            ">
            <input type="date" name="date" required style="
                background: rgba(255,255,255,0.05); border: 1px solid var(--glass-border); padding: 10px; color: white; border-radius: 6px;
            ">
            <textarea name="description" placeholder="Description" style="
                background: rgba(255,255,255,0.05); border: 1px solid var(--glass-border); padding: 10px; color: white; border-radius: 6px; min-height: 80px;
            "></textarea>
            <div style="display: flex; gap: 1rem;">
                <button type="submit" style="
                    background: hsl(var(--accent-secondary)); color: black; border: none; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-weight: 600;
                ">Save Event</button>
                <button type="button" id="btn-cancel" style="
                    background: transparent; border: 1px solid var(--glass-border); color: var(--text-secondary); padding: 8px 16px; border-radius: 6px; cursor: pointer;
                ">Cancel</button>
            </div>
        </form>
    `;const r=e.querySelector("#milestone-form");r.addEventListener("submit",o=>{o.preventDefault();const n=new FormData(r);De({title:n.get("title"),date:n.get("date"),description:n.get("description"),type:"manual",tags:[]}),e.style.display="none",U(t)}),e.querySelector("#btn-cancel").addEventListener("click",()=>{e.style.display="none"})}function U(e){const t=e.querySelector(".timeline-feed"),r=Pe();if(r.length===0){t.innerHTML='<div style="color: var(--text-secondary);">No events recorded.</div>';return}t.innerHTML=r.map(o=>`
        <div class="timeline-item" style="position: relative; margin-bottom: 2rem;">
            <div class="timeline-dot" style="
                position: absolute;
                left: -2.4rem;
                top: 0.2rem;
                width: 12px;
                height: 12px;
                background: hsl(var(--accent-primary));
                border-radius: 50%;
                box-shadow: 0 0 10px hsl(var(--accent-primary));
            "></div>
            <div class="item-date" style="font-size: 0.85rem; color: var(--accent-secondary); margin-bottom: 0.25rem;">
                ${o.date}
            </div>
            <h3 style="font-size: 1.2rem; margin-bottom: 0.5rem; color: var(--text-primary);">${o.title}</h3>
            <p style="color: var(--text-secondary); line-height: 1.6;">${o.description}</p>
        </div>
    `).join("")}const Ne={feed:Le,blindspots:Me,timeline:qe};let f=null,R=null;function C(e){if(e===R)return;const t=document.getElementById("view-container");if(!t)return;f&&f.destroy(),t.innerHTML="",R=e,f=Ne[e],f&&f.init(t),document.querySelectorAll(".nav-tab").forEach(o=>{o.classList.toggle("active",o.dataset.view===e)});const r=document.getElementById("search-input");r&&(r.value="")}document.addEventListener("DOMContentLoaded",()=>{document.querySelectorAll(".nav-tab").forEach(e=>{e.addEventListener("click",()=>{C(e.dataset.view)})}),C("feed")});
