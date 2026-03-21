If you want to contribute or contact me, you can join the fluxer community at https://fluxer.gg/ruF5PHFr <-- The link won't work, I'm making a new server once I make the crate usable with support for all common endpoints. You can DM me on Fluxer `pilkey#0770` if you want to talk, I'm also in the developer server so you can ping me there too.

The `fluxer-api` crate actually has issues (I recently tried to send a message using it but realised I can't add message content to it??), so if anyone wants to make it better please do so, or contact me

The `fluxer-api` crate has moved to https://github.com/PilkeySEK/fluxer-api !

# API Coverage
There is a lot to do
- **Gateway:**
  - **Bot Gateway:** 100% (make an issue if something is missing)
  - **User Gateway:** Most things are the same, but I haven't looked at it exactly, likely more than 90% (PASSIVE_UPDATES missing, which is non-bot users only)
- **HTTP API:** Not a lot... I'm working on it

This is the current support for different APIs (Categories named after [The official Fluxer API documentation sections](https://docs.fluxer.app/api-reference/instance/get-instance-discovery-document)):

✅ = Fully supported

🟡 = Partially supported / In progress

❌ = Not (yet) supported

- Instance ✅
- Admin ❌
- OAuth2 ❌
- Auth ❌
- Channels ✅
- Invites ✅
- Saved Media ✅
- Webhooks 🟡
- Discovery ❌
- Donations ❌
- Gateway (API gateway info endpoint) ❌
- Gifts ❌
- Guilds 🟡
- KLIPY ❌
- Packs ❌
- Premium ❌
- Read States ❌
- Reports ❌
- Search ❌
- Billing ❌
- Tenor ❌
- Users ❌
- Connections ❌
- Themes ❌

# Woah new stuff
So, basically, while trying to cover parts of the API, I am sometimes discovering not yet documented features. Most of them are just not *documented*, but actually do exist in the fluxer code, which is fine. But, some others, like the GUILD_AUDIT_LOG_ENTRY_CREATE (related structs in `model/src/guild/audit_log`) dispatch event are not even in the fluxer codebase yet and instead only in the private `fluxer-v2` repository which is only available to Fluxer visionaries. I'm sadly not a visionary, so I don't fully know the structure of this event. I'm still including it for the future, but you shouldn't rely on it currently (if you happen to be insane enough to use this project seriously).

# Third party code
I have copied code in the following files from project `twilight` (https://github.com/twilight-rs/twilight):
- [model/src/guild/audit_log.rs](./model/src/guild/audit_log.rs)
- [model/src/guild/audit_log/*](./model/src/guild/audit_log) (Everything in that directory)

Copyright (c) 2019 (c) The Twilight Contributors. See [THIRD_PARTY_LICENSES](./THIRD_PARTY_LICENSES) for the full license text.

Note that I have taken some of the structure and ideas (like IDs with markers) from twilight too. You may find some very similar code or structs, which might either be because both twilight and these crates work with an almost identical API or it might be because I have — knowingly or unknowingly — used parts of the twilight code. The list of files are just a list of when I explicitly used their code instead of writing my own. The ISC license actually does not require me to state which parts I copied.
