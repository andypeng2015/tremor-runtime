
```trickle
define operator bucket from grouper::bucket;

define script categorize
script
  let $class = "test";
  let $dimensions = [event.type, event.application];
  let $rate = 1;
  event;
end;

create operator bucket;

create script categorize;

select event from in into categorize;

select event from categorize into bucket;

select event from bucket into out;
```

