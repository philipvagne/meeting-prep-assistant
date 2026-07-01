# Application Structure

Meeting Prep Assistant consists of one primary application window and one separate brief window.

## Main Application

### Home
Purpose:
Access generated meeting briefs.

### Upcoming Meetings
Purpose:
Allow users to manually generate briefs before the automatic generation time.

### Settings
Purpose:
Configure the application.

## Brief Window

Purpose:
Provide a calm, focused reading experience for a single meeting brief.

---

Main Application

├── Home
│   └── Generated Briefs
│
├── Upcoming Meetings
│   └── Manual Brief Generation
│
└── Settings

Separate Window

└── Brief

---

# Main Window

## Purpose

Why does this screen exist?

It exists so that recently generated briefs have a place to live, are easy to see and find and are easy to click on.

---

## Primary User

Who is this screen designed for?

- Everyone

---

## User Goal

What is the user trying to accomplish by opening this screen?

- Finding and clicking on their briefs

---

## Primary Information

What information MUST be visible on this screen?

- A list of recently generated clickable briefs.
- A Navigation Menu

---

## Secondary Information

What information would be useful, but is not required to be immediately visible?

- A small searchbar

---

## Information That Should NOT Be Here

What information should intentionally NOT appear on this screen?

- Everything that i havent mentioned.

---

## Primary Actions

What actions can the user perform from this screen?

- Click on a brief to open it
- Click on anything in the navigation menu to open that window instead of the main window
- Use the searchbar

---

## Default State

What does the screen look like when everything is working normally?

Example:
- No brief generated yet
- One brief waiting
- Multiple briefs waiting

Describe the normal experience.

- A navigation menu
- An empty brief list
- Searchbar

---

## Empty State

What should the user see if there are no briefs?

- A navigation menu
- An empty brief list
- Searchbar

---

## Error State

What should happen if something goes wrong?

Examples:
- Google disconnected
- AI provider missing
- Brief generation failed

- An error message about it that dosnt disapear until the user clicks on a small "x" attached to it, that provides trust and that the user have seen the message

---

## Information Hierarchy

What should the user's eyes naturally look at first?

- Brief List

Second?

- Navigation menu

Third?

- Searchbar

---

## Cognitive Load

How does this screen reduce the user's cognitive load?

- Its pretty much 3 things in this screen with nothing really standing out in a flashy way. 
- The brief list should take up the most amount of space so its naturally easy to see if there is a brief there or not, there is no searching around for it.

---

## Success Criteria

When has this screen successfully done its job?

How do you know this screen has fulfilled its purpose?

- If its easy to understand and simple to use

---

## Future Ideas (Not Version 1)

Any ideas that belong here eventually but should NOT be part of Version 1.

- Not that i can think about right now

---


# Brief View

## Purpose

Why does this screen exist?

- To show the full brief that contains the information about the upcoming meeting.

---

## Primary User

Who is this screen designed for?

- Anyone who wants to read the brief.

---

## User Goal

What is the user trying to accomplish by opening this brief?

- Get a quick little information brief to catch up on what the upcoming meeting is about and what might be talked about

---

## Primary Information

What information MUST be visible?

- Meeting Title + Description + Time
- People attending (If that information exists, otherwise it should be skiped)
- Bulletpoints with information around the meeting, including links to the source of the information as a reference

I think thats it really. 


---

## Secondary Information

What information would be useful but is not immediately required?

- Some way for the user to add their own notes
- I imagine like a subtle button that if a user clicks it they can see things like "AI Provider, Confidence, When the brief was generated etc". But only choose to see it. Its not shown as a default

---

## Information That Should NOT Be Here

What information should intentionally NOT appear?

- As i always say. Nothing else then what ive already mentioned should appear.

---

## Information Hierarchy

When the brief opens...

What should the user read first?

- Meeting Title + Description + Time

Second?

- Bulletpoints with information around the meeting, including links to the source of the information as a reference

Third?

- People attending (If that information exists, otherwise it should be skiped)

---

## Reading Experience

How should reading the brief feel?

Simple, Easy to understand, Well spaced and good typography so its easy to read. 

---

## Trust

How should the application communicate confidence?

- Source Links to reference the information
- I imagine like a subtle button that if a user clicks it they can see things like "AI Provider, Confidence, When the brief was generated etc". But only choose to see it. Its not shown as a default

How should sources be shown?

- Links to reference the information
- I imagine like a subtle button that if a user clicks it they can see things like "AI Provider, Confidence, When the brief was generated etc". But only choose to see it. Its not shown as a default

How should uncertainty be shown?

- With honest in plain text within the brief
- Maybe colorcoded someway, i dont know yet. I probably have to see that to decide

---

## AI Behaviour

What should AI do well?

- Make readability high
- Make the structure good within the brief
- Pretty much provide a good brief
- Highlight where it failed to provide information with good confidence
- I might have missed something here so things can get added or removed

What should AI avoid?

- Everything else

---

## Actions

What actions can the user perform while viewing a brief?

Examples:
- Open source
- Copy brief
- Refresh
- Mark as read
- Delete

(Only include actions you think belong.)

- Copy Brief
- Create own notes
- Click the source links 
- Open the "specifics" as i talked about before "I imagine like a subtle button that if a user clicks it they can see things like "AI Provider, Confidence, When the brief was generated etc". But only choose to see it. Its not shown as a default"
- Mark as read should be automaticly done when the brief is opened
- Delete should be in the main window.
- I would love if the brief opens in a complete seperate window and can move around by itself outside of the app.

---

## Default State

Describe the normal experience.

- You open the brief, you read the brief and get the information you need, you go to the meeting

---

## Empty State

What happens if the brief contains almost no context?

- If that is the case it should state that it was hard to find context on this matter. If there is some emails or documents that might be of interest they should be shown 
as links but with some text that says something like "This might be of interest".

---

## Error State

What happens if the brief cannot be generated?

- There needs to be a message telling the user that it couldnt be generated and why. But in a way so that the user understand, not to much technical terms if possible.

---

## Cognitive Load

How does this screen reduce cognitive load?

- By just keeping it simple, clear, and not provide to much information.

---

## Success Criteria

When has this screen successfully done its job?

- When a user have read the brief and got the information they wanted in a peaceful and clear way.

---

## Future Ideas (Not Version 1)

Ideas for later versions that should not be implemented now.

Not that i can think about right now

---

# Settings

## Purpose

Why does this screen exist?

- Simple, to change settings 

---

## Primary User

Who uses this screen?

- Anyone that want to change the default setting, and in v1 also setup the AI/API.

---

## User Goal

What is the user trying to accomplish here?

- They want to change default behavour and how some functions in the app work 

---

## Sections

What sections should exist?

Example:
- Google Account
- AI Provider
- Notifications
- Application

Should contain
- Google account
- AI provider and being able to change it
- For v1 API Key
- Notifications, being able to change the default timer

---

## Information That Should NOT Be Here

What settings do NOT belong?

- Anything that i didnt mention

---

## Advanced Settings

What settings should be hidden behind an "Advanced" section?

- Probably API key

---

## Default Experience

What should a first-time user see?

- An easy settings window that you understand in under 5 seconds. 

What should a returning user see?

- An easy settings window that you understand in under 5 seconds.

---

## Actions

What can the user do here?

- See what Google account they are logged in to / change Google Account
- See what AI Provider and model they are using / change AI Provider and model
- Put in their API key for choosen AI Provider
- Change the timer for Notifications

---

## Success Criteria

When has this screen done its job?

- When the user easily understand what they can change and how

---

## Future Ideas (Not Version 1)

- Light / Dark mode
- Brief settings ? User could have a bigger impact on what/how information shows
- Turn on and off Gmail and Drive?, Maybe a user just want to fetch information from one off them