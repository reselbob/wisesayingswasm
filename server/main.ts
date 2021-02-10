// @deno-types="./wisesayings.d.ts"
import { serve } from "https://deno.land/std@0.86.0/http/server.ts";
import { get_wise_saying } from "./wisesayings.js";

const env = Deno.env.toObject();

let port = 4040;
if(env.WISESAYING_PORT){
  port = Number(env.WISESAYING_PORT);
};

const server = serve({ hostname: "0.0.0.0", port});
console.log(`HTTP webserver running at ${new Date()}.  Access it at:  http://localhost:${port}/`);

for await (const request of server) {
    let bodyContent = "Your user-agent is:\n\n";
    bodyContent += request.headers.get("user-agent") || "Unknown";
    const saying = get_wise_saying();
    request.respond({ status: 200, body: saying });
  }
