
        div.{resource}__{field}.field {{
            label for="{resource}__{field}" "{field}"
            input  id="{resource}__{field}" type="{html_input_type}" name="{resource}[{field}]" value=^({resource}.{field}()) /
        }}
