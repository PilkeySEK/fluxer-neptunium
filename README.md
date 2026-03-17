If you want to contribute or contact me, you can join the fluxer community at https://fluxer.gg/ruF5PHFr

The `fluxer-api` crate actually has issues (I recently tried to send a message using it but realised I can't add message content to it??), so if anyone wants to make it better please do so, or contact me

The `fluxer-api` crate has moved to https://github.com/PilkeySEK/fluxer-api !

# Woah new stuff
So, basically, while trying to cover parts of the API, I am sometimes discovering not yet documented features. Most of them are just not *documented*, but actually do exist in the fluxer code, which is fine. But, some others, like the GUILD_AUDIT_LOG_ENTRY_CREATE (related structs in `model/src/guild/audit_log`) dispatch event are not even in the fluxer codebase yet and instead only in the private `fluxer-v2` repository which is only available to Fluxer visionaries. I'm sadly not a visionary, so I don't fully know the structure of this event. I'm still including it for the future, but you shouldn't rely on it currently (if you happen to be insane enough to use this project seriously).

# Third party code
I have copied code in the following files from project `twilight` (https://github.com/twilight-rs/twilight):
- [crates/model/src/guild/audit_log.rs](./crates/model/src/guild/audit_log.rs)
- [crates/model/src/guild/audit_log/*](./crates/model/src/guild/audit_log) (Everything in that directory)

Copyright (c) 2019 (c) The Twilight Contributors. See [THIRD_PARTY_LICENSES](./THIRD_PARTY_LICENSES) for the full license text.

Note that I have taken some of the structure and ideas (like IDs with markers) from twilight too. You may find some very similar code or structs, which might either be because both twilight and these crates work with an almost identical API or it might be because I have — knowingly or unknowingly — used parts of the twilight code. The list of files are just a list of when I explicitly used their code instead of writing my own. The ISC license actually does not require me to state which parts I copied.