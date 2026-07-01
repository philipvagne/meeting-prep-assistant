# Product Experience

## Introduction

This document defines how Meeting Prep Assistant should feel to use.

The purpose of this document is not to describe implementation details or technical architecture.

Instead, it captures the intended user experience so future design, UI, UX, prompting, and feature decisions all move toward the same product vision.

If a future feature does not support this experience, it should be reconsidered.

---

# 1. Core Purpose

## In one sentence...

**What problem is Meeting Prep Assistant solving?**

It helps you reduce cognitive load and stress by providing you with a pre meeting brief with the information that you need.

---

## Why does this application exist?

For multiple reasons. It lets you release "mind space" to focus on other things. imagin that you have a meeting at 12:00 and then another one at 14:00. It can be easy to start thinking about those meetings early in the morning. "What are the meetings about?", "Do i need to read up on things about the meeting?", "Who is in the meeting?", these are just some examples that might go trough a person head during the workday. To release stress and cognitive load and let you focus on other work tasks the application exists so that the person can get an automated and AI generated Pre meeting brief 15 minutes (Or the user can just choose when they want it) before the meeting. The application scans the persons calender and then matches the meeting information with similar information that exists in emails and in files to give you a short informational brief, it also contains links to where the information comes from so you can easily just click a link and get right into that document if you want to get the full scope, its up to the person. 

---

## What makes it different from simply opening Google Calendar, Gmail and Drive?

Its automated and runs in the background. this means that you dont manually have to search for the information that you want/need in multiple places. 

---

# 2. First Impression

Imagine someone launches the application for the first time.

## What should they feel within the first 10 seconds?

They should get a soft and cozy/calm feeling. They should see a very simple "first page" UI that is easy to understand and dosnt stress the person out. All the settings and stuff around that should just be in another tab. The main page should be simple, include the briefs and very calm. If there is no briefs generated yet becuase its 8 in the morning and the first meeting is at 10 am, There is no briefs, simple.

---

## What should immediately stand out?

This might be weird. But i think that nothing should stand out. If there is a brief in that has recently been generated thats should "stand out" but in a relaxing way.

---

## What should NOT immediately stand out?

Pretty much everything.

---

# 3. Before a Meeting

Imagine a meeting starts in 15 minutes.

## What should happen?

You get a notification (Native windows notification it looks like is the only possible one.), You click it and the application opens up on your screen

---

## What should the application already have done?

Generated the brief and show it so that it can be opened.

---

## What should the user NOT need to do?

Well, the user should not have to do anything more then click the notification to open the application (Or whatever way they wanna open the application, maybe from tray, maybe alt + tab, its up to the user), then click on the brief and watch it open.

---

# 4. During the Brief

## What information matters most?

- Meeting title + description + time
- Who is gonna attend the meeting if that information exists
- Some generated information from emails/documents that gives you a heads up on what the meeting is about
- Links to where that information came from

I think thats it

---

## What information is nice to have?

- There could be some more technical information around the brief and how its generated but it should not show in the brief, its should be a choice in someway if the user wanna see it or not
- This is not really information but maybe someway the user can write some own notes in the brief if they want to.

This is a hard question, i think that this one will evolve during the process. Its easier to see and feel whats good, whats bad, what do i need, what dont i need when you are creating it and building it i think

---

## What information is unnecessary noise?

Everything that i havent mentioned.

---

## When should the AI say "I don't know"?

When its not confident on the information it can provide, if it fits with the meeting. 

---

# 5. Trust

## What makes the user trust the generated brief?

I think that the links to the information provided is trustworthy, becuase that is the AI providing a source.
That could be enough. Becuase even with an application like this i think that most users already have some idea of what the meeting is about, so when they 
start reading a brief, see the information and links, i think that is enough

---

## How should sources be presented?

100%, without them the trust goes down alot. You always want to be able to check the information provided if you want to.

---

## How should uncertainty be communicated?

Well this is a hard one. It has to be in an honest and professional way. 
It could communicate the information with links as usuall but with some extra text that says that this should be looked at by the person one extra time becuase of uncertanties. 
this is one that could change and need some more thinking on from my side i believe.

---

# 6. Personality

Imagine Meeting Prep Assistant were a colleague.

## How would you describe them?

Laidback, hanging around in the background quietly, there when you need them, chill to hang with on the free time, gives you good answers to your questions.

---

## Which words describe the application?

Examples:

- Calm
- Quiet
- Helpful
- Professional
- Reliable
- Fast
- Intelligent
- Minimal
- Friendly
- Invisible

I would say all of them. But they all have there value in different phases.

Always:
- Calm
- Friendly
- Quiet
- Helpful

Running in the background:
- Minimal
- Invisible

Creating a brief an UI browsing:
- Fast
- Reliable

Putting the brief together
- Intelligent

Presenting the brief
- Professional


---

## Which words should NEVER describe the application?

Examples:

- Loud
- Distracting
- Bloated
- Pushy
- Confusing
- Flashy
- Annoying

Straight up all of them, If any of those are real. I have failed.

---

# 7. UI Philosophy

## What should every screen prioritize?

A calm "workspace" but with professionalism involved. They should be easy to navigate and easy to understand. Smooth and calm colors and font. Smooth transistions. 
This might sound weird, but every screen should not have that many things happen in them. 

---

## What should the user never feel when looking at the UI?

Stressed and confused. 

---

## How much information should be visible by default?

Honestly, not much at all.

---

## What should always be one click away?

If you are on the main page, the brief.
If you are on the settings page, the brif. But thats 2 clicks, change to main page, click the brief.
So there is not anything that should be "ALWAYS" one click away. But assuming that users spend 99% of their application time o the main page, its the brief.

---

# 8. Notifications

## When should notifications appear?

Id set a default timer at 15 minutes. But I want it to be a setting where the user can decide the timer for themself. 

---

## What should a notification accomplish?

Just give you a notice that a meeting is coming up and a brief is waiting for you. Also clicking it should open the app.

---

## What should notifications never become?

A distraction and irritating thing.

---

# 9. AI Philosophy

## What should AI do?

Generate the briefs and present them.
Im also thinking if possible search for the information in Gmail and Drive, If it could look for information with a "semantic mind" that would be the best option.

---

## What should AI never do?

Probably everything else that dosnt include "Generate the briefs and present them."

---

## What should happen when AI has weak context?

It should be honest about it to the user. Not try to come up with anything that isnt the truth, not stop working. Just be honest.

---

## How much should AI infer?

Not more then it has to

---

# 10. Success

## When does the user close the application feeling successful?

When the application helped them out with what they wanted from it, its not harder then that.

---

## Finish this sentence:

> "After using Meeting Prep Assistant I feel..." - good that i didnt had to think about that upcoming meeting all morning.

---

# 11. Failure

## Finish this sentence:

> "I would uninstall the application if..." - it dosnt work in a good and reliable way

---

## What mistakes are completely unacceptable?

Giving bad briefs.
Leaking information from emails/drive/calendar/application

---

# 12. Product Principles

## Finish this sentence:

> "The user should never have to..." - manually prepare for a meeting ever again

---

## Finish this sentence:

> "The application should always..." - give me what i want from it

---

## Finish this sentence:

> "Every feature should..." - be easy to understand

---

## Finish this sentence:

> "Every notification should..." - just give me a heads up

---

## Finish this sentence:

> "Every AI-generated sentence should..." - feel as human as possible

---

## Finish this sentence:

> "We will intentionally NOT..." - leak any information

---

# 13. Product Identity

If someone asked you what Meeting Prep Assistant is...

## Describe it in one sentence.

An automated application that runs in the background and give you a brief so that you easily can prepare for a meeting without having to think about it.

---

## Describe it in one paragraph.

Not today im not.

---

## What emotions should someone feel after using it?

They shouldnt really think about it. But relieved maybe

---

## What emotions should they NEVER feel?

Stressed and embarresed (If the brief is wrong)

---

# 14. Future Vision

Imagine the application is two years old.

## What has remained exactly the same since version 1?

That its still calm and easy to understand.

---

## What do you hope has improved dramatically?

Well, thats hard to answer when i dont know how V1 will end up

---

## If users recommend Meeting Prep Assistant to a friend, what do you hope they say?

That it have helped them alot when it comes to not have to think about meetings alot in advance and instead focus on other things. That its a great way to get some small fast information right before getting into a meeting and be up to speed.

---

# Closing Thoughts

Is there anything else about the experience that doesn't fit into the questions above?

Write it here.