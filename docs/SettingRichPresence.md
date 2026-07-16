> ## Documentation Index
> Fetch the complete documentation index at: https://docs.discord.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Setting Rich Presence

> Update Discord Rich Presence from your application to display activity information.

export const InboxIcon = props => <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none"><path fill="currentColor" fill-rule="evenodd" d="M5 2a3 3 0 0 0-3 3v14a3 3 0 0 0 3 3h14a3 3 0 0 0 3-3V5a3 3 0 0 0-3-3H5ZM4 5.5C4 4.67 4.67 4 5.5 4h13c.83 0 1.5.67 1.5 1.5v6c0 .83-.67 1.5-1.5 1.5h-2.65c-.5 0-.85.5-.85 1a3 3 0 1 1-6 0c0-.5-.35-1-.85-1H5.5A1.5 1.5 0 0 1 4 11.5v-6Z" clip-rule="evenodd" /></svg>;

export const MagicDoorIcon = props => <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none"><path fill="currentColor" d="M9 10a1 1 0 0 1 1 1v2a1 1 0 0 1-2 0v-2a1 1 0 0 1 1-1Z" /><path fill="currentColor" fill-rule="evenodd" d="M13 1a9 9 0 0 1 9 9v8a3 3 0 0 1-3 3H5a3 3 0 0 1-3-3v-8a9 9 0 0 1 9-9h2Zm5.5 15a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3ZM12 8.22a4 4 0 1 0-8 0v9.5a1 1 0 0 0 1.24.97l5.72-1.43c.6-.15 1.04-.7 1.04-1.34v-7.7Zm5.68.26a.73.73 0 0 0-1.36 0l-.18.48a2 2 0 0 1-1.18 1.18l-.48.18a.73.73 0 0 0 0 1.36l.48.18a2 2 0 0 1 1.18 1.18l.18.48a.73.73 0 0 0 1.36 0l.18-.48a2 2 0 0 1 1.18-1.18l.48-.18a.73.73 0 0 0 0-1.36l-.48-.18a2 2 0 0 1-1.18-1.18l-.18-.48ZM14.5 4a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3Z" clip-rule="evenodd" /></svg>;

export const UserIcon = props => <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none"><path fill="currentColor" d="M12 10a4 4 0 1 0 0-8 4 4 0 0 0 0 8ZM11.53 11A9.53 9.53 0 0 0 2 20.53c0 .81.66 1.47 1.47 1.47h.22c.24 0 .44-.17.5-.4.29-1.12.84-2.17 1.32-2.91.14-.21.43-.1.4.15l-.26 2.61c-.02.3.2.55.5.55h11.7a.5.5 0 0 0 .5-.55l-.27-2.6c-.02-.26.27-.37.41-.16.48.74 1.03 1.8 1.32 2.9.06.24.26.41.5.41h.22c.81 0 1.47-.66 1.47-1.47A9.53 9.53 0 0 0 12.47 11h-.94Z" /></svg>;

## Overview

Rich Presence allows you to display detailed information about what players are doing in your game. Users can see this information in their Discord profile and friends list and use it to join their friends' games with Game Invites.

### Prerequisites

Before you begin, make sure you have:

* [Set up the Discord Social SDK](/developers/discord-social-sdk/getting-started)
* Connected to Discord with a valid client instance

<Info>
  This feature requires the **Default Presence Scopes** (`openid` and `sdk.social_layer_presence`).
  Use [`Client::GetDefaultPresenceScopes`] when configuring your OAuth2 flow.
  See the [OAuth2 Scopes guide](/developers/discord-social-sdk/core-concepts/oauth2-scopes) for details on all available scopes.
</Info>

[`Client::GetDefaultPresenceScopes`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1Client.html#a7648bd1d2f7d9a86ebd0edb8bef12b5c

***

## Understanding Rich Presence

Rich Presence allows you to display detailed information about players' actions in your game. Users can see this information in various places in Discord, including:

* User profiles
* Friend lists
* Server member lists

<Info>
  Let's talk about the naming of some Discord primitives first. Rich Presence, aka "Activity", can be thought of as the "current activity of a user" and is represented by the [`Activity`] class in the SDK and [in our gateway events](/developers/events/gateway-events#activity-object). This is not to be confused with [Discord Activities](/developers/activities/overview), which are embedded games that can also set and display rich presence.
</Info>

Each [`Activity`] contains fields that describe the following:

| Field                | Description              | Purpose                                                                                                     |
| -------------------- | ------------------------ | ----------------------------------------------------------------------------------------------------------- |
| `name`               | Game or app name         | Displayed in the user's profile                                                                             |
| `type`               | Activity type            | What the player is doing (e.g., "Playing", "Watching", "Listening")                                         |
| `details`            | What the player is doing | Main activity description (e.g., "Playing Capture the Flag")                                                |
| `state`              | Their current status     | Secondary status (e.g., "In Queue", "In Match, "In a group")                                                |
| `party`              | Party information        | Shows party size and capacity (e.g., "2 of 4")                                                              |
| `timestamps`         | Activity duration        | Shows elapsed or remaining time                                                                             |
| `assets`             | Custom artwork           | Game/map thumbnails and character icons                                                                     |
| `secrets`            | Join/spectate tokens     | Enable [Game Invite](/developers/discord-social-sdk/development-guides/managing-game-invites) functionality |
| `supportedPlatforms` | Platform flags           | Control where join buttons appear                                                                           |

See below for examples of how to set these fields in your code.

<Info>
  While we support multiple [`ActivityTypes`], games should use `ActivityTypes::Playing` for `type`. The SDK automatically associates the activity with your game, so by default fields like `name` show your game's name. If you'd like to customize the displayed name, see [Setting the Application Name](/developers/discord-social-sdk/development-guides/setting-rich-presence#setting-the-application-name).
</Info>

### Customizing Rich Presence

When displayed in Discord, Rich Presence has three main components:

```
Playing "Your Game Name"          <- Line 1: Game name (automatic)
Capture the Flag | 2 - 1          <- Line 2: Details field
In a group (2 of 3)               <- Line 3: State + Party info
```

You can control how lines 2 and 3 are rendered in Discord, here's the breakdown:

* Line 1, `Playing "game name"` is powered by the name of your game (or application) on Discord.
* Line 2, `Capture the flag | 2 - 1` is powered by the `details` field in the activity, and this should generally try to describe what the *player* is currently doing. You can even include dynamic data such as a match score here.
* Line 3, `In a group (2 of 3)` describes the *party* the player is in. "Party" is used to refer to a group of players in a shared context, such as a lobby, server, team, etc. The first half, `In a group` is powered by the state field in the activity, and the second half, `(2 of 3)` is powered by the party field in the activity and describes how many people are in the current party and how big the party can get.

This diagram visually shows the field mapping:

<img src="https://mintcdn.com/discord/eCuA2RUd7jZAoO2Y/images/rich-presence/legend.png?fit=max&auto=format&n=eCuA2RUd7jZAoO2Y&q=85&s=e4f5578bd804892645024322bb89c0bd" alt="Graphical representation of the legend for rich presence details" width="3060" height="702" data-path="images/rich-presence/legend.png" />

<Info>
  For tips on designing Rich Presence, take a look at the [Rich Presence best practices guide](/developers/rich-presence/best-practices).
</Info>

***

## Setting an Invite Image

The Rich Presence invite image appears when invites are sent for a 3rd party game or app using the Discord Social SDK. After uploading an invite image for your app, you can see a preview of it to the right (under "IRL Invite Image Example").

<img src="https://mintcdn.com/discord/eCuA2RUd7jZAoO2Y/images/rich-presence/invite-image.png?fit=max&auto=format&n=eCuA2RUd7jZAoO2Y&q=85&s=c0d6da1093da84212296daffea680bf0" alt="Rich Presence invite image in app settings" width="1536" height="645" data-path="images/rich-presence/invite-image.png" />

***

## Uploading Assets

While integrating Rich Presence, you'll likely want to upload custom art assets for your app. For all Rich Presence assets, it's highly recommended to make them 1024 x 1024.

To add custom assets for Rich Presence, navigate to your app's settings and click Rich Presence on the left-hand sidebar. On the [Art Assets page](https://discord.com/developers/applications/select/rich-presence/assets), you can upload two different types of assets.

<img src="https://mintcdn.com/discord/eCuA2RUd7jZAoO2Y/images/rich-presence-invite-image.webp?fit=max&auto=format&n=eCuA2RUd7jZAoO2Y&q=85&s=639d47015744c265cf7ea89c372223f3" alt="Rich Presence invite image in app settings" width="2058" height="705" data-path="images/rich-presence-invite-image.webp" />

Up to 300 custom assets can be added to your app for later use when setting Rich Presence for a Discord user. These assets can be anything that help orient others to what a user is doing inside of your Activity or 3rd party game.

If you need more than 300 custom assets or want to use images stored somewhere else, you can also [specify an external URL](/developers/events/gateway-events#activity-object-activity-asset-image) as long it still has the proper dimensions and size. Unlike uploaded assets, external URLs also support GIF, animated WebP, and AVIF.

<Info>
  For tips on choosing assets, take a look at the [Rich Presence best practices guide](/developers/rich-presence/best-practices#have-interesting-expressive-art).
</Info>

When uploading Rich Presence assets, **the asset keys will automatically be changed to lowercase**. You can see this reflected in your app's settings after saving a newly uploaded asset, and you should keep it in mind when referencing any asset keys in your code.

Once you've uploaded these assets, you can use the asset key to reference them in your code when [Setting Assets in Rich Presence](/developers/discord-social-sdk/development-guides/setting-rich-presence#setting-assets).

<img src="https://mintcdn.com/discord/eCuA2RUd7jZAoO2Y/images/rich-presence/asset-images.png?fit=max&auto=format&n=eCuA2RUd7jZAoO2Y&q=85&s=aa08786199295bc3f71bd18d3e6ae35f" alt="Rich Presence assets in app settings" width="1527" height="723" data-path="images/rich-presence/asset-images.png" />

***

## Setting Details and State

Here's a simple example setting the details and state of a Rich Presence activity:

```cpp theme={"system"}
// Create a new activity
discordpp::Activity activity;
activity.SetType(discordpp::ActivityTypes::Playing);
activity.SetDetails("Battle Creek");
activity.SetState("In Competitive Match");

// Update the presence
client->UpdateRichPresence(activity, [](discordpp::ClientResult result) {
  if (result.Successful()) {
    std::cout << "✅ Rich presence updated!\n";
  }
});
```

***

## Setting the Application Name

By default, Rich Presence displays your application's name as it's registered on Discord (the top line of Rich Presence, e.g. `Playing "My Game: Chapter Two"`). You can override this displayed name by setting the `name` field on the activity with [`Activity::SetName`] before calling [`Client::UpdateRichPresence`]:

```cpp theme={"system"}
// Override the displayed application name (top line of Rich Presence)
activity.SetName("My Game: Chapter Two");
```

<Tip>
  Registered application names are restricted to a limited character set. Using [`Activity::SetName`] gives you more flexibility over exactly what text is displayed, including characters that aren't permitted in your registered application name.
</Tip>

***

## Setting Timestamps

You can include timestamps in your Rich Presence to display a live timer, such as how long a player has been in their current activity or how much time is left in a timed match. Timestamps are [`ActivityTimestamps`] set as Unix timestamps in seconds.

The direction of the timer is determined by which field you set:

* Set the **start** time only to count **up** and show elapsed time (e.g. `12:34 elapsed`).
* Set the **end** time to count **down** and show the remaining time (e.g. `12:34 left`).

### Count up (elapsed time)

To show how long the player has been in the activity, set [`ActivityTimestamps::SetStart`] to a time in the past, such as the current time. The timer counts up from that point.

```cpp theme={"system"}
// Counting up: elapsed time since the activity started
discordpp::ActivityTimestamps timestamps;
// time(nullptr) returns the current Unix time in seconds, so the timer starts now
timestamps.SetStart(time(nullptr));
activity.SetTimestamps(timestamps);
```

### Count down (time remaining)

To show how long until a timed activity ends, set [`ActivityTimestamps::SetEnd`] to a time in the future. The timer counts down to that point.

```cpp theme={"system"}
// Counting down: time remaining until the activity ends (one hour from now)
discordpp::ActivityTimestamps timestamps;
// time(nullptr) is the current Unix time in seconds; + 3600 sets the end one hour from now
timestamps.SetEnd(time(nullptr) + 3600);
activity.SetTimestamps(timestamps);
```

<Tip>
  Setting the **end** time is what makes the timer count down. If you only set the **start** time, the timer counts up instead. You can set both — `start` still controls the elapsed reference point, but as long as `end` is set the timer displays the remaining time.
</Tip>

***

## Setting Assets

Once you've uploaded assets to your app, you can reference them using their **asset key** in your code to set custom artwork in Rich Presence. Here's an example of an asset with the key of "map-mainframe", "tank-avatar", and "invite-cover-image":

```cpp theme={"system"}
// Setting Activity Assets
discordpp::ActivityAssets assets;
assets.SetLargeImage("map-mainframe");
assets.SetLargeText("Mainframe");
assets.SetSmallImage("tank-avatar");
assets.SetSmallText("Tank");
assets.SetInviteCoverImage("invite-cover-image"); // Used for Game Invites
activity.SetAssets(assets);
```

<Info>
  If you need more than 300 custom assets or want to use images stored somewhere else, you can also [specify an external URL](/developers/events/gateway-events#activity-object-activity-asset-image) as long it still has the proper dimensions and size.

  Unlike uploaded assets, external URLs also support GIF, animated WebP, and AVIF.
</Info>

***

## Setting Field URLs

You can set URLs for `details`, `state`, `assets.large_image` and `assets.small_image` in Rich Presence. When present, these URLs will make the corresponding image/text into clickable links.

```cpp theme={"system"}
activity.SetState("Playing on Mainframe");
activity.SetStateUrl("https://example.com/maps/mainframe");
activity.SetDetails("Rank #1337 in global leaderboard");
activity.SetDetailsUrl("https://example.com/leaderboard/global");

discordpp::ActivityAssets assets;
assets.SetLargeImage("map-mainframe");
assets.SetLargeText("Mainframe");
assets.SetLargeUrl("https://example.com/maps/mainframe");
assets.SetSmallImage("tank-avatar");
assets.SetSmallText("Tank");
assets.SetSmallUrl("https://example.com/classes/tank");

activity.SetAssets(assets);
```

***

## Setting Buttons

You can add up to two custom buttons to a player's Rich Presence. Each button has a label and a URL, making them a direct call to action for anyone viewing the presence — link out to your game's store page, website, or community server.

<CodeGroup>
  ```cpp C++ theme={"system"}
  discordpp::ActivityButton buyButton;
  buyButton.SetLabel("Buy On Steam!");
  buyButton.SetUrl("https://store.steampowered.com/app/AppID/GameName/");

  discordpp::ActivityButton communityButton;
  communityButton.SetLabel("Join the Community!");
  communityButton.SetUrl("https://discord.gg/your-discord-url-or-id");

  activity.AddButton(buyButton);
  activity.AddButton(communityButton);
  ```

  ```csharp Unity theme={"system"}
  ActivityButton buyButton = new ActivityButton();
  buyButton.SetLabel("Buy On Steam!");
  buyButton.SetUrl("https://store.steampowered.com/app/AppID/GameName/");

  ActivityButton communityButton = new ActivityButton();
  communityButton.SetLabel("Join the Community!");
  communityButton.SetUrl("https://discord.gg/your-discord-url-or-id");

  activity.AddButton(buyButton);
  activity.AddButton(communityButton);
  ```

  ```cpp Unreal theme={"system"}
  UDiscordActivityButton* BuyButton = NewObject<UDiscordActivityButton>();
  BuyButton->Init();
  BuyButton->SetLabel("Buy On Steam!");
  BuyButton->SetUrl("https://store.steampowered.com/app/AppID/GameName/");

  UDiscordActivityButton* CommunityButton = NewObject<UDiscordActivityButton>();
  CommunityButton->Init();
  CommunityButton->SetLabel("Join the Community!");
  CommunityButton->SetUrl("https://discord.gg/your-discord-url-or-id");

  Activity->AddButton(BuyButton);
  Activity->AddButton(CommunityButton);
  ```
</CodeGroup>

<Tip>
  Buttons are only visible to **other users** — you cannot see buttons on your own Rich Presence. To test that your buttons are working, use a second account or ask a friend to view your profile.
</Tip>

***

## Configuring Status Text

By default, Rich Presence will display the game's name in the user's status text. You can override this behavior by setting a status display type.

```cpp theme={"system"}
// uses the game's name in the status text (default)
activity.SetStatusDisplayType(discordpp::StatusDisplayTypes::Name);

// uses the activity's state field in the status text
activity.SetStatusDisplayType(discordpp::StatusDisplayTypes::State);

// uses the activity's details field in the status text
activity.SetStatusDisplayType(discordpp::StatusDisplayTypes::Details);
```

***

## Setting Party and Join Secret

You can also include party details and a join secret in your Rich Presence to power Game Invites. Check out the [Game Invites guide](/developers/discord-social-sdk/development-guides/managing-game-invites) for more information.

```cpp theme={"system"}

// Setting Party Details
discordpp::ActivityParty party;
party.SetId("party1234");
party.SetCurrentSize(1);
party.SetMaxSize(5);
activity.SetParty(party);

// Setting Join Secret details
discordpp::ActivitySecrets secrets;
secrets.SetJoin("your-join-secret");
activity.SetSecrets(secrets);
```

***

## Setting Supported Platforms

You can set the supported platforms for your game in Rich Presence. This will control where the join buttons appear in Discord.

If you only want the join button to appear on desktop, you can set the supported platforms like this:

```cpp theme={"system"}
activity.SetSupportedPlatforms(discordpp::ActivityGamePlatforms::Desktop);
```

See the `ActivityGamePlatforms` enum for all supported platforms.

## Rich Presence Without Authentication

<Warning>
  Rich Presence via RPC (Remote Procedure Call) will only work with a running Discord desktop client. It does not support mobile, console or web clients.
</Warning>

Unlike most other features of the Discord Social SDK, **Rich Presence can be set without authentication**. Instead of
using [`Client::Connect`] to authenticate with Discord, you can use Rich Presence functionality by directly communicating
with a running Discord desktop client through RPC (Remote Procedure Call).

### Requirements

* Discord desktop client must be running on the user's machine
* Your application must be registered with Discord and have a valid Application ID

This direct approach makes Rich Presence integration much simpler for developers who only need basic presence
functionality while Discord desktop clients are running.

### Setting Up Direct Rich Presence

To use Rich Presence without authentication, simply:

1. Set your application ID using [`Client::SetApplicationId`]
2. Configure your activity details
3. Call [`Client::UpdateRichPresence`]

```cpp theme={"system"}
auto client = std::make_shared<discordpp::Client>();

// Set the application ID (no Connect() call needed)
client->SetApplicationId(APPLICATION_ID);

// Configure rich presence details
discordpp::Activity activity;
activity.SetType(discordpp::ActivityTypes::Playing);
activity.SetState("In Competitive Match");
activity.SetDetails("Rank: Diamond II");

// Update rich presence
client->UpdateRichPresence(
    activity, [](const discordpp::ClientResult &result) {
      if (result.Successful()) {
        std::cout << "🎮 Rich Presence updated successfully!\n";
      } else {
        std::cerr << "❌ Rich Presence update failed";
      }
    });
```

***

## Next Steps

Now that you've set up Rich Presence, you might want to explore:

<CardGroup cols={3}>
  <Card title="Managing Game Invites" href="/developers/discord-social-sdk/development-guides/managing-game-invites" icon={<InboxIcon />}>
    Allow players to invite friends to join their game session or party.
  </Card>

  <Card title="Managing Lobbies" href="/developers/discord-social-sdk/development-guides/managing-lobbies" icon={<MagicDoorIcon />}>
    Bring players together in a shared lobby with invites, text chat, and voice comms.
  </Card>

  <Card title="Design Guidelines for Rich Presence" href="/developers/discord-social-sdk/design-guidelines/status-rich-presence" icon={<UserIcon />}>
    Best practices for Rich Presence UI/UX.
  </Card>
</CardGroup>

Need help? Join the [Discord Developers Server](https://discord.gg/discord-developers) and share questions in the `#social-sdk-dev-help` channel for support from the community.

If you encounter a bug while working with the Social SDK, please report it here:  [https://dis.gd/social-sdk-bug-report](https://dis.gd/social-sdk-bug-report)

***

## Change Log

| Date           | Changes                                     |
| -------------- | ------------------------------------------- |
| June 24, 2026  | Clarified count-up vs count-down timestamps |
| June 5, 2026   | Added Setting the Application Name section  |
| March 31, 2026 | Added Setting Buttons section               |
| March 17, 2025 | Initial release                             |

[`Activity`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1Activity.html#ae793d9adbe16fef402b859ba02bee682

[`Activity::SetName`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1Activity.html#a59f9a63a8b105946d0c9838f3e643ae2

[`ActivityTimestamps`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1ActivityTimestamps.html#a0ff108aac69639c18f1669994e459ee2

[`ActivityTimestamps::SetEnd`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1ActivityTimestamps.html#ab6478ec46860feb64174da3eb1063aaa

[`ActivityTimestamps::SetStart`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1ActivityTimestamps.html#a5bb6a6bc243fedb954ae82d6c4ef3542

[`ActivityTypes`]: https://discord.com/developers/docs/social-sdk/namespacediscordpp.html#a6c76a8cbbc9270f025fd6854d5558660

[`Client::Connect`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1Client.html#a873a844c7c4c72e9e693419bb3e290aa

[`Client::SetApplicationId`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1Client.html#ad452335c06b28be0406dab824acccc49

[`Client::UpdateRichPresence`]: https://discord.com/developers/docs/social-sdk/classdiscordpp_1_1Client.html#af0a85e30f2b3d8a0b502fd23744ee58e
