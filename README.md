# `FoodLoss.csv` Parser

Code is self-explanatory.

Structure of the file should be:

```
m49code            string
country            string
region             string | null
cpc_code           string
commodity          string
year               integer
loss_percentage    float
activity           string | null
food_supply_stage  string | null
cause_of_loss      string | null
```

...with <=3 irrelevant columns at the end.

