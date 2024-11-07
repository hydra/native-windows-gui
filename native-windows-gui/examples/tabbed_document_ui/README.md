# Tabbed Document UI example

Example for evaluating GUI framework suitability for creating productivity-style desktop apps.

Implementing this forces the developer to learn about some key points of the framework:
* Global state (config).
* Events or reactive system.
* Dynamic tab bars.
* Dynamic containers.
* Common widgets: (Labels, Buttons, Checkboxes, Drop-downs).
* Layout & Styling: (Alignment, Spacing/Padding, Grids).
* Native integration (file dialogs, etc).
* Composability and reusability.

## Requirements

- Main window
    - [x] native look and feel titlebar with native close, minimize, maximize control
    - [x] re-sizable, contents adjust to fit.
- Toolbar
    - 'Home' button
        - [x] which when clicks shows a home tab.
        - [x] if the home tab is already open, it should switch to it.
	- 'Open' button
	    - [x] When clicked, shows a native file selector dialog which allows the user to choose a file.
        - [x] If an unsupported file is selected, show a native error dialog.
        - [x] When a file is opened, a tab appears and depending on the file type, it shows different content in the tab. (e.g. '.txt' text shows the 'Text' tab, '.bmp' shows the 'Image' tab).
	- 'New' button
	    - [x] When clicked the 'New' tab is shown, see below.
	- [ ] Language dropdown, choose between at least 2 languages (e.g. English and Spanish).
        - [ ] Changing the language should cause all UI text to be immediately displayed in the selected language without requiring a restart. 
- Tab bar
	- [x] When all the tabs won't fit in the window, there must be some controls to allow them all to be selected, e.g. `<` and `>` buttons, or `V` dropdown, or scrollable.
	- [x] Selecting a tab changes the content area below the tab bar.
	- [ ] Must be obvious which tab is selected when there are only two tabs.
	- [x] Each tab should be closable (e.g. an `X` button on the tab or right-click on tab to show a context menu with `Close`)
	- [x] When a tab is closed, the next most recently used tab is made active.
- Tab content
	- [x] Displays the content for the tab.
	- [x] Each tab content must maintain it's state, without expensive re-loads/refreshing of the state, no re-loading of files.
    - [ ] Scroll bars should appear if the content does not fit the window.
	- Tabs
		- 'Home' tab
			- [x] Shows a welcome message.
			- [x] Shows a checkbox with the message 'Open on startup', see 'state items' below.
		- 'New' tab
			- [x] a form is shown with 3 main controls , each with a label, in a grid with labels on the left. Below the form an OK button should be present.
				- Name - text entry, ideally with placeholder text, no default name.
				- Type - dropdown, initially nothing selected, choose between Text or Bitmap.
					- dropdown should always appear, correctly and allow all elements to be chosen even if the window is resized.
				- Directory - non-editable path with a button to show a native directory selector to be used, that when selected shows the path.
			- [x] Title of the new tab is 'New'
			- [x] Multiple 'new' tabs are allowed, each with their own state.
            - Tab state
			    - [x] The form field values.
			- When OK is pressed
				- [x] the tab name should be updated to the name of the file.
				- [x] a file should be created with the appropriate extension.
				- [x] it's content should be displayed in the same tab, see tab content below.
				- [x] there should be no visible removal and insertion of any new tab.
				- [x] tab ordering must be preserved.  e.g. given tabs 'File1, New, File2' pressing 'Ok' on `New` should result in tabs 'File1, File3, File2', not 'File1, File2, File3'
		- 'Text' tab, displays a 'text' document.
			- [x] Filename must appear in tab.
			- [x] Show text file content in an editor.
			- [x] Content must be loaded in a thread or async, in the background.
			- Tab State
				- [ ] Maintain text selection.
				- [ ] Maintain caret position.
			- Info sidebar with a grid of key/value items
				- [x] File path
				- [ ] Selection information
		- 'Image' tab, displays an 'image' document.
			- [x] Filename must appear in tab.
			- [x] Shows the image.
            - [ ] Image is top-left justified.
            - [x] Image is scaled-up to fit window, aspect ratio must be preserved.
            - [ ] Do not allow image to be scaled down.
			- [ ] If too big to fit in the window, scrollbars must be present to allow panning
			- [x] Content must be loaded in a thread or async, in the background.
			- Tab State
				- [ ] Maintain X/Y coordinates of last click of anywhere on the image.
			- Info sidebar with a grid of key/value items
				- [x] File path
				- [x] Last-clicked X/Y coordinate information
                - [x] Image size. (width, height).
- Application state must be loaded on program start, and saved as appropriate.
	- State items
		- [x] 'Open home tab on startup', boolean, initially true.
			- [x] If true, open the 'Home' tab on startup.
		- [x] 'List of currently open files' (ignore `New` tabs), list of absolute filenames, initially empty.
			- [x] Create a tab for each file on startup.
- Documents
  - 'text' - the text file.
  - 'image' - the image file. 
- Architecture
    - [ ] Code should be written in such a way that multiple-developers can work on different aspects of the codebase without creating merge-conflicts. i.e. use modules, avoid tight-coupling, good compile-time dependencies, etc.
    - [x] The application itself must own the documents (images, text), not the tabs themselves.
    - [x] When the last tab for a document is closed, the document should be dropped/closed.
- Bonus points
	- [ ] Native look and feel controls.
	- [x] Some way of closing all the tabs in one go (e.g. "Close all" button on toolbar).
    - [ ] Add 'Display in window' on tab context menu which when clicked displays the document in a new window with no tab bar, and where the window title is the name of the file.
	- [ ] Multiple tabs for the same document, e.g. right click a tab, click 'Duplicate'.  Changes in one tab are reflected in the other.
	- [ ] Status bar, showing some active-tab-specific state, e.g. last click location on image tabs. changes when changing tabs. (e.g. IDEs often show line number, offset, and selected line/character counts in the status bar).
	- [ ] When two tabs are open, where the file name names of the document are the same, but the directory the file is in is different, show enough of the path to be able to distingush the two tabs.
		- e.g. for `/tmp/foobar/file.txt` and `/tmp/barfoo/file.txt` instead of (`file.txt` & `file.txt`) show (`foobar/file.txt` & `barfoo/file.txt`)
		- doing this forces the tab system to be able to access other tab names and change them all, dynamically, when one tab is added or when one is updated.
	- [ ] Draggable divider between sidebar and content.
    - [ ] Tests for individual components.
    - [ ] Integration/Behavioral tests.
