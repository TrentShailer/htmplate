// deno-lint-ignore-file
// deno-fmt-ignore-file
// @ts-nocheck
// @ts-self-types="./fetch.d.ts"
let TOKEN_KEY="token";async function fetch(method,url,additionalHeaders,body){var a=new Headers;if(additionalHeaders)for(var e of additionalHeaders)a.append(e[0],e[1]);body&&a.append("content-type","application/json");additionalHeaders=localStorage.getItem(TOKEN_KEY);additionalHeaders&&a.append("Authorization",additionalHeaders);let t=null;body&&(t=JSON.stringify(body));additionalHeaders=await self.fetch(url,{method:method,body:t,headers:a}).catch(()=>new Response(null,{status:500}));if(additionalHeaders.ok){body=additionalHeaders.headers.get("Authorization");body&&localStorage.setItem(TOKEN_KEY,body);
// deno-lint-ignore no-explicit-any
let a={};try{a=await additionalHeaders.json();
// deno-lint-ignore no-empty
}catch{}return{status:"ok",body:a}}if(401===additionalHeaders.status)return{status:"unauthorized"};if(400<=additionalHeaders.status&&additionalHeaders.status<500){let a={problems:[]};try{a=await additionalHeaders.json();
// deno-lint-ignore no-empty
}catch{}return{status:"clientError",problems:a.problems??[]}}return{status:"serverError"}}export{TOKEN_KEY,fetch};