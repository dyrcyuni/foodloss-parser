# `FoodLoss.csv` Parser

Code is self-explanatory.

Structure of the file should be:

```
m49code            String
country            String
region             String | null
cpc_code           String
commodity          String
year               u32
loss_percentage:   f32
activity           String | null
food_supply_stage  String | null
cause_of_loss      String | null
```

...with <=3 irrelevant columns at the end.

