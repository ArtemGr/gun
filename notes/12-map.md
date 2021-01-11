
☑ Experiment with ObservableHQ graphs ⇒ https://observablehq.com/@artemgr/rusty-gun-a-story  
☐ Make a larger map in `rusty-gun-a-story`  
→ ☑ Narrow down to major goals ⇒ Bottom-up development; modularity; transfer of knowledge in OSS  
☐ Gource in the README  
cf. https://superuser.com/questions/556029/how-do-i-convert-a-video-to-gif-using-ffmpeg-with-reasonable-quality  
cf. https://github.com/cyburgee/ffmpeg-guide  
→ ☐ Upload the Gource log also, making it accessible from ObservableHQ, in case we'd want to add dynamic JS visualizations later  
☐ Consider making a list of projects using the rusty GUN. Should it be in the README? Can we feature a random project/contributor/backer in the README (similar to crate-of-the-week in TWiR)? cf. https://github.com/GiraffeKey/dahlia  
☐ List contributors in Change Log, cf. https://github.com/Geal/nom/blob/master/CHANGELOG.md

☐ Benchmarks  
→ ☐ Different tasks should have different benchmarks  
→ ☐ Improve the way benchmarks are communicated in a task  
→ ☐ Improve the way benchmarks are measured and represented  
→ ☐ Graphics and stats  
→ ☐ Benchmarks can also be useful for research and orientation, cf. “categorize the issues and compare the impacts of the bounties, for example, bounties for bug fixes versus bounties for feature additions” at https://www.freelancer.com/contest/tell-us-about-the-open-source-bounty-ecosystem-1873760-byentry-47325551

☐ Experiment with bottom-up (leader-leader) bounty-driven development  
→ ☑ Phase one, the Telegram group ⇒ Positive feedback, small traction  
→ ☑ Daily top up ⇒ Added to TasksDb  
→ ☒ List of known bounty kinds ⇒ I suspect this is “disruptive to routine tasks”, should focus on task creation, selection and acquisition first, and then return to the *bonus lottery* later on  
→ ☑ “take a break” experiment ⇒ https://github.com/ArtemGr/bounty/issues/5; https://www.freelancer.com/contest/take-a-break-go-out-and-make-a-picture-1873762/entries; https://www.upwork.com/ab/applicants/1347469384349327360/job-details  

☐ RLS bountry experiment (https://www.reddit.com/r/rust/comments/kur3vn/rls_bounty_583_to_fix_stuck_on_indexing/)

☐ Consider the competition format,  
cf. https://vk.com/durovschallenge, https://t.me/contests_ru, https://t.me/contest, https://codeforces.com/

☐ Explore some of the literature adjucent to motivation streams  
→ ☑ https://arxiv.org/pdf/1805.09850.pdf Ruohonen Allodi 2018  
→ → ☐ https://www.sciencedirect.com/science/article/abs/pii/S0268401216305412  
→ → ☐ https://www.sciencedirect.com/science/article/abs/pii/S1361372314704634  
→ → ☐ https://alarcos.esi.uclm.es/DocumentosWeb/2017-Journal%20of%20Systems%20and%20Software-Garc%C3%ADa.pdf  
→ → ☐ “jit20157a - impact of openness on multi-sided platforms.pdf”  
→ ☑ Organic Design  
→ ☐ Turn the Ship Around ⇒ 32%  
→ → ☑ Discuss with Chet  
→ ☐ Flirt: Afterword  
→ ☐ Brain Tingles  
→ ☐ What the F  
→ ☐ Man's search for meaning  
→ ☐ Louder Than Words  
→ ☐ Pre-Suasion

☐ Experiment with Qualitative Interviewing  
→ ☑ Create a separate MarkDown space  
→ ☑ Transcript the first set of interviews  
→ ☐ Second round, self-wishing and the Happy New Year  
→ ☐ Finish working through the book  
→ ☐ Grab the book highlights  
→ ☐ Revisit the book highlights  
→ ☑ Assemble a list of people whom we can ask about GUN  
→ ☐ Interview with Aelita  
☐ Consider creating a Telegram channel in order to allow for convenient video messaging  
→ ☐ See if Telescope is supported in Telegram groups  
→ ☑ Telescope promotes good video ecology: short videos are easier to discard and retry, easier to watch and to index, allow for chunked communication and dialogue. Question is, can we transfer this somehow to Discord? A bot that would copy the Telegram Telescope videos to Discord? And should we bother? ⇒ Discord can embed short (under 8 MB) libx264 mp4 videos, which seems good enough, though maybe we should collect the ways to record these  
☐ Figure how/if the Qualitative Interviewing can factor into the bounty-driven development  
→ ☑ File the idea of embedded QI  
→ ☑ Consider the format of public podcasts ⇒ Known downsides: time synchronization, TMI, hard to get different opinions from different people, postprocessing not included  
→ ☑ Experiment with video attachments on GitHub  
→ ☐ Play with the idea of (incentivized) transcripts

☐ Look at gossipsub, as it might affect the design and the wishlists around the p2p layers  
→ ☐ [Gossipsub-v1.1 Evaluation Report](https://gateway.ipfs.io/ipfs/QmRAFP5DBnvNjdYSbWhEhVRJJDFCLpPyvew5GwCCB4VxM4)  
→ ☐ https://blog.ipfs.io/2020-05-20-gossipsub-v1.1/

# work group

@mhelander on SEA: “my part was to make SEA to work, how it's integrated into Gun is more @marknadal super skills I think...”, continued at https://discord.com/channels/612645357850984470/612645357850984473/793023656795045920

@mimiza: “I only implemented SEA.certify(). It's just a small part in SEA. I don't even know SEA better than @mhelander”

Mark: “@mhelander @jabis @mmalmi @rogowski @mimiza @go1dfish @Dletta probably have the best understanding of various internal parts”

#docs-wiki can be used for pubsub
