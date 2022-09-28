This implementation is copied from [https://github.com/yewstack/yew], then:
* Extract parts that can be shared between implementations to todomvc_shared.
* todomvc_shared always have id in TodoEntry
* The rest of todomvc example stayed in yew_non_keyed
* Moved fields that is UI related from todomvc_shared to yew_non_keyed
* Modified yew_non_keyed to use TodoEntry.id instead of indices.
* Field named `editing` in each entry, now a single field in the app state: `editing_id`
