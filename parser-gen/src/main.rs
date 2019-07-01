#![allow(unused_imports)]
mod cdl;
//mod cdl_parser_gen;
//mod cdl_parser_gen;
//mod common;
//mod xmlparser;


fn main() {
  println!("Hello, world!");
  let start = std::time::Instant::now();

  let lexer = cdl::lexer::Lexer::new();
  let res = lexer.lex(_SCRIPT.to_string());
  let dur = start.elapsed();
  match res {
    Ok(r) => println!("{:?}", r.len()),
    Err(e) => println!("Something went wrong {:?}", e.0),
  }
  println!("{}, {}, {}, {}",dur.as_secs(), dur.as_millis(), dur.as_micros(), dur.as_nanos());
//  println!("{:?}",res.unwrap());

}


const _SCRIPT: &str = "title \"Empower - Peter's test\"

config hub {
  hub: 991
  table accounts = crmdata.Accounts_2      //accounts custom table
  table survey = p1049059.responseid  //relationship survey
  table contacts = p1049153.responseid
  table healthCheck = p1049245.responseid      //healthcheck survey
  // table cases = am.CASE
  table respondent = p1049059.respondent
  table revenue = crmdata.Historical_Revenue  // historical revenue custom table.
  table ejournal = crmdata.eJournal
  table accessRules = crmdata.accessRules
  table salesHier = crmdata.SalesRegion

  variable singleChoice AD {
    label: \"Act Dash\"
    table: survey:
    option code {
     code: \"Full\"
      score:1
      label: \"Completes\"
    }
    option code {
      code: \"Partial\"
      score: 2
      label: \"Partials\"
    }
    option code {
      code: \"No\"
      score: 3
      label: \"Error\"
    }
    // value: IIF(survey:status='complete',\"Full\",IIF(survey:status='incomplete' OR survey:status='screened',\"Partial\",IIF(respondent:smtpstatus='messagesent' AND NOT(survey:status='complete' OR survey:status='screened' OR survey:status='incomplete'),\"No\")))
    value: IIF(survey:status='complete',\"Full\",IIF(survey:status='incomplete',\"Partial\",IIF(respondent:smtpstatus='messagesent' AND NOT(survey:status='complete' OR survey:status='incomplete'),\"No\")))
  }

  //accounts --> revenue
  relation oneToMany rel3 {
     primaryKey: accounts:accountID
     foreignKey: revenue:accountID
  }

  // accounts --> Health
  relation oneToMany rel2 {
    primaryKey: accounts:AccountID
    foreignKey: healthCheck:shortAccountID

  }
  // accounts --> contact db --> survey
  relation oneToMany rel4 {
    primaryKey: accounts:AccountID
    foreignKey: contacts:shortAccountID
  }
  //accounts --> eJournal
  relation oneToMany rel5 {
     primaryKey: accounts:eJournalAccountNumber
     foreignKey: ejournal:compid
  }

  relation oneToMany {
    primaryKey: salesHier:SalesRegion
    foreignKey: accounts:SalesRegionNumber

  }

  // additional User access stuff
  userProperty claim myRole {
    joinKey: accessRules:Username
    value: accessRules:Role
  }

  userProperty claim myFullName {
    joinKey: accessRules:Username
    value: accessRules:UnderscoredName
  }
  userProperty claim myRegion{
    joinKey: accessRules:Username
    value: accessRules:SalesRegion
  }
  userProperty claim myRegionName{
    joinKey: accessRules:Username
    value: accessRules:Region
  }

}

config report cr {
  currentPeriod: InMonth(survey:interview_start,-11,0)
  previousPeriod: InMonth(survey:interview_start,-21,-12)
  currentPeriodTC: InYear(healthCheck:interview_start,-1,0)
  previousPeriodTC: InYear(healthCheck:interview_start,-2,-1)
}

layoutArea toolbar {
  filter singleselect {
    label: \"Show\"
    option radio {
      label: \"My Portfolio\"
      value: accounts:AccountOwner=@currentUser.myFullName AND @currentUser.myRole=\"AccountOwner\"
    }
    option radio {
      label: \"My Team\"
      value: accounts:SalesRegion=@currentUser.myRegionName
      selected: true
    }
  }


}

config report cr {

formatter date dateFormat {
inputFormat: \"YYYYMM\"
formatString: \"MMM\"
}

formatter date dateForm {
inputFormat: \"YYYYMM\"
formatString: \"MMM YY\"
}

formatter number floatNumber {
numberDecimals: 1
}

formatter date date12 {
locale:en
shortForm: false
}
formatter value valueFMT {
emptyValue: \" \"
}
formatter number formatterID {
numberDecimals : 0
//prefix : \"$ \"
decimalSeparator : \".\"
integerSeparator : \" \"
shortForm : true
}
// formatter compose cmp {
//   formatters: formatterLTR
// }
formatter number formatterLTR {
numberDecimals : 2
decimalSeparator : \".\"
emptyValue: \"-\"
}

formatter number formatterLTRtable {
numberDecimals : 1
decimalSeparator : \".\"
emptyValue: \"-\"
}
formatter number formatterRR {
numberDecimals : 0
decimalSeparator : \".\"
shortForm : true
postfix : \" %\"
}

formatter color formatterColor{
thresholds: #009900 >=9 , #b34700 >=7, #b30000 >=0
}
formatter color formatterColor2{
thresholds: #0099AA >=9 , #b347AA >=7, #b300AA >=0
}
formatter number metricFormat {
numberDecimals: 1
decimalSeparator: \".\"
integerSeparator: \",\"
shortForm: false
}

formatter number responsesFormat {
numberDecimals: 0
shortForm: false
}

formatter color riskStringFormatter {
thresholds: Unknown = 0 , Low = 1, Medium = 2, High = 3
}

formatter color riskBgColorFormatter {
thresholds: #23C813 >= 9, #FFAB00 >= 7, #ff0000 >= 0
}
formatter color risk {
thresholds: #23C813 >= 9, #FFAB00 >= 7, #ff0000 >= 0
}

formatter color backgroundColor {
thresholds: #e8f8e0 >= 9, #ffeed6 >= 7, #fedfe2 >= 0
}
formatter color valueColor {
thresholds: #388e3c >= 9, #ff6d00 >= 7, #d40000 >= 0
}
formatter color transparent {
thresholds: rgba(0,0,0,0) >= 9, rgba(0,0,0,0) >= 7, rgba(0,0,0,0) >= 0
}
// formatter color valueCases {
//   thresholds: #ff0000 >=1, #31363e >=0
// }
// formatter color backgroundColorFormatter {
//   thresholds: #e8f8e0 >= 8.5, #ffeed6 >= 6.5, #fedfe2 >= 0
// }
formatter color valueColorFormatter{
thresholds: #5ba35d >= 8, #ffa156 >= 6.5, #dd3435 >= 0
}
formatter objectProperty textPicker {
property: text
}

formatter color riskColorFormatter{
thresholds:  #FA5263 >= 99%, #FFBD5B >=49%, #82D854 >0%
}

formatter color gaugeColorFormatter{
thresholds:  #82D854 >= 99%, #FFBD5B >=49%, #FA5263 >0%
}
ltrTarget: 9

//  view metric metrics {
//     valueColorFormatter: valueColor
//     fontSize:large
//     backgroundColorFormatter: transparent
//   }
//  view metricWithChange metrics {
//    backgroundColorFormatter: backgroundColor
//    valueColorFormatter: valueColor
//    fontSize:small
//    roundCorners:true
// }
}

page \"Welcome\" {
widget title {
layout column {
tile value {
value: \"Welcome, \" +  @currentUser.givenname
}
tile value {
value: \"Your role: \" + @currentUser.myRole
}
tile value {
value: \"Your Region: \" + @currentUser.myRegionName
}
}
}
}

page \"My Portfolio\" {
access rules {
rule claim {
name: \"myRole\"
value: \"AccountOwner\"
}
}
widget title {
layout column {
tile value {
value: @currentUser.givenname + \"'s portfolio\"
}
}
}

widget accountList {
label: \"Accounts\"
table: accounts:
sortColumn: accountName
//sortOrder: accending
navigateTo: \"Account\"
size: large
hierarchy: accounts:ParentAccountID

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small
//roundCorners:true
}

column hierarchy accountName {
label: \"Account Name \"
value: accounts:AccountName
rowHeader: true
//format: customEmpty
//align: right

}

column metric ltr {
label: \"Client View\"
value: average(score(survey:Q1))
view: metrics
format: formatterLTRtable
target: 9
align: center
}


column metric hh {
label: \"Internal View\"
value: average(score(healthCheck:Q2))
view: metrics
format: formatterLTRtable
target: 9
align: center
}

column value accountMan {
label: \"Account Owner\"
value: accounts:AccountOwner
}

column value revRisk {
label: \"Revenue Risk \" //Churn Risk
value: @cp.revenueRiskValue
align: center
format:riskStringFormatter
}
column value rev2017 {
label: \"2017 rev ($)\"
value: sum(revenue:AnnualRevenue, revenue:year=2017)
format: formatterID
align: right
}
column value rev2016 {
label: \"2016 rev\"
value: sum(revenue:AnnualRevenue, revenue:year=2016)
format: formatterID
align: right
}
column value revDiff {
label: \"Rev diff\"
value: sum(revenue:AnnualRevenue, revenue:year=2017) - sum(revenue:AnnualRevenue, revenue:year=2016)
format: formatterID
align: right
}
column value revDiffPercentage {
label: \"Rev diff\"
value: ((sum(revenue:AnnualRevenue, revenue:year=2017) - sum(revenue:AnnualRevenue, revenue:year=2016))/sum(revenue:AnnualRevenue, revenue:year=2017))*100
format: formatterRR
align: right
}
column value tickets {
label: \"eJournal total\"
value: sum(ejournal:closedTickets)
}
column value ticketsOpen {
label: \"eJournal Open\"
value: sum(ejournal:openTickets)
}

// column value case2 {
//   label: \"Cases\"
//   value: COUNT(cases:CaseId)
//   align: center
//   format: responsesFormat

// }
column value responses {
label: \"Responses\"
format: valueFMTnses
value: COUNT(survey:responseid,(survey:status='complete')) //OR survey:status=\"incomplete\"

align: center
}

column value rate {
label: \"Response Rate\"
value: (COUNT(survey:responseId,(survey:status=\"complete\" OR survey:status=\"incomplete\"))*100) / COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")
format: formatterRR
align: center
}
column value noResp {
label: \"No Response\"
value: COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")-COUNT(survey:responseid,survey:status=\"Complete\")
align: center
}
}

}

page \"My Team\" {
access rules {
rule claim {
name: \"myRole\"
value: \"Manager\"
}
}
widget title {
layout column {
tile value {
value: @currentUser.givenname + \"'s team portfolio\"
}
}
}
widget markdown {
size: large
markdown: \"# Check out how your Team is doing \"
}
widget accountList {
label: \"Accounts\"
table: accounts:
sortColumn: accountName
//sortOrder: accending
navigateTo: \"Account\"
size: large
hierarchy: accounts:ParentAccountID

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small
//roundCorners:true
}

column hierarchy accountName {
label: \"Account Name \"
value: accounts:AccountName
rowHeader: true
//format: customEmpty
//align: right

}

column metric ltr {
label: \"Client View\"
value: average(score(survey:Q1))
view: metrics
format: formatterLTRtable
target: 9
align: center
}


column metric hh {
label: \"Internal View\"
value: average(score(healthCheck:Q2))
view: metrics
format: formatterLTRtable
target: 9
align: center
}

column value accountMan {
label: \"Account Owner\"
value: accounts:AccountOwner
}

column value revRisk {
label: \"Revenue Risk \" //Churn Risk
value: @cp.revenueRiskValue
align: center
format:riskStringFormatter
}
column value rev2017 {
label: \"2017 rev ($)\"
value: sum(revenue:AnnualRevenue, revenue:year=2017)
format: formatterID
align: right
}
column value rev2016 {
label: \"2016 rev\"
value: sum(revenue:AnnualRevenue, revenue:year=2016)
format: formatterID
align: right
}
column value revDiff {
label: \"Rev diff\"
value: sum(revenue:AnnualRevenue, revenue:year=2017) - sum(revenue:AnnualRevenue, revenue:year=2016)
format: formatterID
align: right
}
column value revDiffPercentage {
label: \"Rev diff\"
value: ((sum(revenue:AnnualRevenue, revenue:year=2017) - sum(revenue:AnnualRevenue, revenue:year=2016))/sum(revenue:AnnualRevenue, revenue:year=2017))*100
format: formatterRR
align: right
}
column value tickets {
label: \"eJournal total\"
value: sum(ejournal:closedTickets)
}
column value ticketsOpen {
label: \"eJournal Open\"
value: sum(ejournal:openTickets)
}

// column value case2 {
//   label: \"Cases\"
//   value: COUNT(cases:CaseId)
//   align: center
//   format: responsesFormat

// }
column value responses {
label: \"Responses\"
format: valueFMTnses
value: COUNT(survey:responseid,(survey:status='complete')) //OR survey:status=\"incomplete\"

align: center
}

column value rate {
label: \"Response Rate\"
value: (COUNT(survey:responseId,(survey:status=\"complete\" OR survey:status=\"incomplete\"))*100) / COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")
format: formatterRR
align: center
}
column value noResp {
label: \"No Response\"
value: COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")-COUNT(survey:responseid,survey:status=\"Complete\")
align: center
}
}
}

page \"KPIs\" {



// widget kpi {
//   label: \"NPS\"
//   size: small
//   tile kpi {
//     label:\"NPS\"
//     value: NPS(survey:Q1)*100
//     //target: 25
//     min: -100
//     max: 100
//     format:formatterLTR
//     targetFormat:formatterLTR
//     gaugeColorFormat:kpiColorFormatter  // valueColor
//     tile value {
//       label: \"Responses\"
//       value: count(survey:Q1)
//       //max: count(survey:responseid, @currentPeriodFilter)
//       max: count(survey:responseid)
//       format: integer
//     }
//     tile value {
//       label: \"Yearly change\"
//       value: average(score(survey:Q1)) -average(score(survey:Q1))
//       //value: average(score(survey:Q1),@currentPeriodFilter) -average(score(survey:Q1),@previousPeriodFilter)
//       format:formatterLTR
//     }
//   }
// }


widget kpi {
label: \"NPS\"
size: small
tile kpi {
label:\"NPS\"
value: NPS(survey:Q1, @cr.currentPeriod)*100
target: 20
min: -100
max: 100
format:formatterLTR
targetFormat:formatterLTR
gaugeColorFormat:gaugeColorFormatter  // valueColor
tile value {
label: \"Responses\"
value: count(survey:Q1, @cr.currentPeriod)
max: count(survey:responseid)
// value: count(survey:Q1, @cr.currentPeriodFilter)
// max: count(survey:responseid, @cr.currentPeriodFilter)
format: integer
}
//    tile value {
//    label: \"Yearly change\"
//   value: average(score(survey:Q1),@cr.currentPeriodFilter)-average(score(survey:Q1),@cr.previousPeriodFilter)
//  format:formatterLTR
//  }
}
}

widget kpi {
label: \"Team  Satisfaction\"
size: small
tile kpi {
label:\"AVG\"
value: average(score(survey:Q4), @cr.currentPeriod)
min: 0
max: 10
format:formatterLTR
targetFormat:formatterLTR
gaugeColorFormat: kpiColorFormatter
tile value {
label: \"Responses\"
value:count(survey:responseid,survey:status=\"Complete\" AND @cr.currentPeriod)
max: count(survey:responseid)
format: integer
}
tile value {
label: \"Yearly change\"
//value: average(score(survey:Q4)) -average(score(survey:Q4))
value: average(score(survey:Q4),@cr.currentPeriod) -average(score(survey:Q4),@cr.previousPeriod)
format:formatterLTR
}
}
}

widget kpi {
label: \"Technology  Satisfaction\"
size: small
tile kpi {
label:\"AVG\"
value: average(score(survey:Q7), @cr.currentPeriod)
min: 0
max: 10
format:formatterLTR
targetFormat:formatterLTR
gaugeColorFormat: kpiColorFormatter       //valueColor
tile value {
label: \"Responses\"
value:count(survey:responseid,survey:status=\"Complete\" AND @cr.currentPeriod)
max: count(survey:responseid)
format: integer
}
tile value {
label: \"Yearly change\"
value: average(score(survey:Q7),@cr.currentPeriod) -average(score(survey:Q7),@cr.previousPeriod)
format:formatterLTR
}
}
}

widget kpi {
label: \"Likelihood to Renew\"
//should be renew Q, target 8
size: small
tile kpi {
label:\"AVG\"
value: average(score(healthCheck:Q2), @cr.currentPeriodTC)
target: 8
min: 0
max: 10
format:formatterLTR
targetFormat:formatterLTR
gaugeColorFormat: riskBgColorFormatter
tile value {
label: \"Responses\"
value:count(healthCheck:responseid,healthCheck:status=\"Complete\" AND @cr.currentPeriodTC)
max: count(healthCheck:responseid)
format: integer
}

}
}


widget responseRate {
label: \"Survey responses status\"
tile statuses {
breakBy: survey:AD
value: count(survey:)
chart: bar
}

}

widget portfolioBreakdown {
label: \"NPS Breakdown by Role\"
size: small
navigateTo:\"Accounts\"
category: contacts:ContactRole
segment: survey:NPSSegment
value: count(survey:responseId)
percent: off

}

// widget trend {
//      size: medium
//         timeline: CalendarMonth(survey:interview_start)
//       dateFormat: dateFormat
//         format:floatNumber
//         minValue: 0
//         maxValue: 10
//         palette: \"#86ABE2\",\"#F9BF00\",\"#F18500\",\"#EF6300\",\"#F30000\",\"#AA0010\",\"#C0C0C0\"
//         series value s1 {
//            label: \"LTR, avg\"
//            dot: true
//            value: average(score(survey:Q1))
//      }
//      series value s2 {
//       type: 'monotone'
//         label: \"Satisfaction with technology \"
//       value: average(score(survey:Q4))
//      }
// }


widget recentResponses \"yy\" {
label: \"What our customers saying\"
showHeader: true
navigateTo: \"Response\"
// sortOrder: ''
view comment fff {
lines: 3
}

size: medium
table: survey:
view metric metrics {
valueColorFormatter: valueColor
fontSize:large
backgroundColorFormatter: transparent
}
column response \"x1\" {
sortBy: footer
footer: survey:interview_end
header: (((survey:FirstName + ' ') + survey:LastName ) + \" - \") + accounts:AccountName
comment: survey:Q2
commentFormat:commentFormat

}
column metric ltr2 {
label: \"LTR\"
value: average(score(survey:Q1))//, survey:NPSSegment=\"promoter\")

target: 10
view:metrics
}
}

widget recentResponses \"IV\" {
label: \"Account owners recent responses\"
size: medium
navigateTo: \"AOResponse\"
showHeader: true
view comment fff {
lines: 3
}
table: healthCheck:
view metric metrics {
valueColorFormatter: valueColor
fontSize:large
backgroundColorFormatter: transparent

}

column response \"x11\" {
label: \"Account and Account Owner\"
sortBy: footer
footer: healthCheck:interview_start
header: accounts:AccountName
comment: accounts:AccountOwner
commentFormat:commentFormat
}

column metric ltr21 {
label: \"Likely to Renew\"
value: average(score(healthCheck:Q2))
target: 10
view:metrics
}
}

widget topAccounts {
label: \"High value accounts latest LTR\"
table: accounts:
size: medium
sortColumn: main
navigateTo:\"Account\"
hierarchy: accounts:ParentAccountID
view metricWithChange metrics {
valueColorFormatter: valueColor
backgroundColorFormatter:transparent
fontSize:large
}
column accounts main {
label:\"Account Name\"
accountName: accounts:AccountName
revenue:SUM(accounts:AnnualAccountValue)
value:SUM(accounts:AnnualAccountValue)
}

column metric ltr {
value: average(score(survey:Q1),@cr.currentPeriodFilter)
previous: average(score(survey:Q1),@cr.previousPeriodFilter)
format: formatterLTR
target: @cr.ltrTarget
view:metrics

}
}

}

page \"Risk Analysis\" {

widget portfolioBreakdown \"REX\"{
label: \"Revenue at Risk, next 6 months\"
size: medium
navigateTo: \"Accounts\"
category:IIF(InMonth(accounts:RenewalDate, 0,6) , CalendarMONTH(accounts:RenewalDate))
categoryFormat: dateForm
segment:IIF(count(healthCheck:responseid,true,accounts:)>0, (IIF(average(SCORE(healthCheck:Q2),true,accounts:)>=9, \"Safe\",IIF(average(SCORE(healthCheck:Q2),true,accounts:)>=5, \"Medium\", \"High\"))), \"Unknown\")
//value: count(survey:responseid)
value: sum(accounts:TotalAccountValue)
format:currencyDefaultFormatter
palette:@cr.paletteM
}


}



page \"Accounts\" {

widget search {

layoutArea: \"header\"    // will be deprecated at some point soon
source search account {
table: accounts:
value: accounts:AccountName
navigateTo: \"Account\"
iconType: \"account\"     // this is a temporary way to set correct item icon in suggestion list (before we figure out how to set up suggestion items view)
}

source search contact {
table: contacts:
value: contacts:client_first_name
navigateTo: \"Contact\"
}
}

widget accountList {
label: \"Accounts\"
table: accounts:
sortColumn: accountName
//sortOrder: accending
navigateTo: \"Account\"
size: large
hierarchy: accounts:ParentAccountID

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small
//roundCorners:true
}

column hierarchy accountName {
label: \"Account Name \"
value: accounts:AccountName
rowHeader: true
//format: customEmpty
//align: right

}

column metric ltr {
label: \"Client View\"
value: average(score(survey:Q1))
view: metrics
format: formatterLTRtable
target: 9
align: center
}


column metric hh {
label: \"Internal View\"
value: average(score(healthCheck:Q2))
view: metrics
format: formatterLTRtable
target: 9
align: center
}

column value accountMan {
label: \"Account Owner\"
value: accounts:AccountOwner
}

column value revRisk {
label: \"Revenue Risk \" //Churn Risk
value: @cp.revenueRiskValue
align: center
format:riskStringFormatter
}
column value rev2017 {
label: \"2017 rev ($)\"
value: sum(revenue:AnnualRevenue, revenue:year=2017)
format: formatterID
align: right
}
column value rev2016 {
label: \"2016 rev\"
value: sum(revenue:AnnualRevenue, revenue:year=2016)
format: formatterID
align: right
}
column value revDiff {
label: \"Rev diff\"
value: sum(revenue:AnnualRevenue, revenue:year=2017) - sum(revenue:AnnualRevenue, revenue:year=2016)
format: formatterID
align: right
}
column value revDiffPercentage {
label: \"Rev diff\"
value: ((sum(revenue:AnnualRevenue, revenue:year=2017) - sum(revenue:AnnualRevenue, revenue:year=2016))/sum(revenue:AnnualRevenue, revenue:year=2017))*100
format: formatterRR
align: right
}
column value tickets {
label: \"eJournal total\"
value: sum(ejournal:closedTickets)
}
column value ticketsOpen {
label: \"eJournal Open\"
value: sum(ejournal:openTickets)
}

// column value case2 {
//   label: \"Cases\"
//   value: COUNT(cases:CaseId)
//   align: center
//   format: responsesFormat

// }
column value responses {
label: \"Responses\"
format: valueFMTnses
value: COUNT(survey:responseid,(survey:status='complete')) //OR survey:status=\"incomplete\"

align: center
}

column value rate {
label: \"Response Rate\"
value: (COUNT(survey:responseId,(survey:status=\"complete\" OR survey:status=\"incomplete\"))*100) / COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")
format: formatterRR
align: center
}
column value noResp {
label: \"No Response\"
value: COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")-COUNT(survey:responseid,survey:status=\"Complete\")
align: center
}
}
}

page account \"Account\" {

mainTable: accounts:
widget search {

layoutArea: \"header\"    // will be deprecated at some point soon
source search account {
table: accounts:
value: accounts:AccountName
navigateTo: \"Account\"
iconType: \"account\"     // this is a temporary way to set correct item icon in suggestion list (before we figure out how to set up suggestion items view)
}

source search contact {
table: contacts:
value: contacts:client_first_name
navigateTo: \"Contact\"
}
}

widget title {
table: accounts:
layout column {
tile value c {
value: accounts:accountName
}
}
}

widget summary {
size: large
table: accounts:
hierarchy: accounts:ParentAccountID

tile metric {
label: \"LTR Average\"
value: average(score(survey:Q1), @currentPeriod)
target: 8
}


tile metric {
label: \"Account Owner View\"
value: average(score(healthCheck:Q2))
target: 8
}


tile risk {
label: \"Revenue Risk\"
// need to tune formatting  add calculation
showThermometer: false
value: @cp.revenueRiskValue
textValue: IIF(@cp.revenueRiskValue=3, \"High Risk\", IIF(@cp.revenueRiskValue=2, \"Medium Risk\", IIF(@cp.revenueRiskValue=1, \"Low Risk\", \"Unknown\")))
min: 1
max: 3
target: 0
renewal: accounts:RenewalDate
revenue: sum(revenue:AnnualRevenue, revenue:year=2017)
//backgroundColorFormatter: riskColorFormatter
}

tile responseRate {
invites: COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\" AND @cr.currentPeriod)
responses: COUNT(survey:responseId,(survey:status=\"complete\" OR survey:status=\"incomplete\") AND @cr.currentPeriod)
showThermometer: false
}

// tile casesStatus {
//    label: \"Cases\"
//   open: COUNT(cases:CaseId)
//   overdue: 0
// }
}

widget metricsBeta {

view metricWithBar metric {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
chartColorFormatter: chartColorFormatter
fontSize:small
roundCorners:false
}

label: \"Relationship Survey Metrics\"
size: large

tile header {

item title {
value: 'KPI'
}

item title {
value: 'Average'
align:left
}
// item title {
//   value: 'Base'
//   align: left
// }
item title {
value: 'Comments'
align:left
}
}

tile row 1 {
item value {
value: 'LTR'
rowHeader: true
}

item metric {
value: average(score(survey:Q1))
target: 8
view: metric
}

// item value {
//   value: count(survey:Q1)
//   align: right
// }


item value {
value: count(survey:Q2)
align: right
}
}

tile row x {
item value {
value: 'Satisfaction with relatiohship'
rowHeader: true
}

item metric {
value: average(score(survey:Q4))
view: metric
target: 8
}

item value {
value: count(survey:Q6)
align: right
}

item value {
value: ''
}

}

tile row 3 {
item value {
value: 'Satisfaction with technology'
rowHeader: true
}

item metric {
value: average(score(survey:Q7))
view: metric
target: 7
}

item value {
value: count(survey:Q7)
align: right
}

//  item value {
//   value: count(survey:Q8)
//   align: right
// }

item value {
value: ''
}

}
tile row 4 {
item value {
value: 'Product is scalable'
rowHeader: true
}

item metric {
value: average(score(survey:Q9.1))
view: metric
target: 7
}

item value {
value: count(survey:Q9.1)
align: right
}

item value {
value: ''
}
}

tile row 5 {
item value {
value: 'Confirmit adds value'
rowHeader: true
}

item metric {
value: average(score(survey:q3))
view: metric
target: 7
}

item value {
value: count(survey:Q9.1)
align: right
}

item value {
value: ''
}
}
}

widget contactSurveys {
label: \"Internal View\"
table: healthCheck:
sortColumn: surveyDate
sortOrder: ascending
size: large
navigateTo:\"Account Owner Response\"

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small
//roundCorners:true
}

column value accountMan {
label: \"Account Owner\"
value: accounts:AccountOwner
}

column date surveyDate {
label: \"Date\"
value: healthCheck:interview_start
}

column metric ltr {
label: \"LTR estimate\"
value: average(score(healthCheck:Q1))

view: metrics
target: 9
align: center
}

column metric ltr2 {
label: \"Renewal\"
value: average(score(healthCheck:Q2))
view: metrics
target: 9
align: center
}
column metric ltr3 {
label: \"Growth Potential\"
value: average(score(healthCheck:Q5))
view: metrics
target: 9
align: center
}

column metric ltr4 {
label: \"Dependency on Services\"
value: average(score(healthCheck:Q11))
view: metrics
target: 9
align: center
}
// column value case2 {
//   label: \"Cases\"
//   value: COUNT(cases:CaseId)
//   align: center
//   format: responsesFormat
// }

column value comments {
label: \"Comments: how to keep the customer\"
value: healthCheck:Q4
}
}


widget contactSurveys {
label: \"All Surveys\"
table: survey:
sortColumn: surveyDate
sortOrder: ascending
size: large
navigateTo: \"Response\"

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small
//roundCorners:true
}

column value status {
label: \"Status\"
value: survey:status
}


column date surveyDate {
label: \"Submitted\"
value: survey:interview_start
format: date11
}

column metric ltr {
label: \"LTR\"
value: average(score(survey:Q1))
view: metrics
target: 9
align: center
}

column value s5 {
label: \"Email\"
value: survey:email
}

column value comments {
label: \"Comments\"
value: survey:Q2
}
}

widget contactList hg {
size: large
label:\"Contacts \"
table: contacts:
sortColumn: name
sortOrder: descending
navigateTo: \"Contact\"

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small

//roundCorners:true
}

view metric ccc {
valueColorFormatter:valueCases
}
column value name {
label: \"Name\"
value: (contacts:FirstName + \" \") + contacts:LastName
}
column value company {
label:\"Company\"
value: contacts:AccountName
}
column value role {
label: \"Role\"
value: contacts:ContactRole
}

column metric ltr {
label: \"LTR\"
value: average(score(survey:Q1))
view:metrics
target:9
}
column value  {
label: \"First Invite SentStatus\"
value: LAST(respondent:FirstEmailedDate, respondent:FirstEmailedDate)
format: date11
asign: center
}

column value  {
label: \"Status\"
value: LAST(respondent:smtpStatus, respondent:smtpStatusDate)
asign: center
}

column value lastResponseZZ {
label: \"Last Invite sent\"
value: LAST(respondent:smtpStatusDate, respondent:smtpStatusDate)
format:date11
asign: center
}
column value lastResponse {
label: \"Last Interview started\"
value: LAST(survey:interview_start, survey:interview_start)
format:date11
asign: center
}


column value comments {
label: \"Comments\"
value: LAST(survey:Q2,survey:interview_start)
}
}

}

page contact \"Contact\" {

widget search {

layoutArea: \"header\"    // will be deprecated at some point soon
source search account {
table: accounts:
value: accounts:AccountName
navigateTo: \"Account\"
iconType: \"account\"     // this is a temporary way to set correct item icon in suggestion list (before we figure out how to set up suggestion items view)
}

source search contact {
table: contacts:
value: (contacts:client_first_name + \" \") + contacts:client_last_name
navigateTo: \"Contact\"
}
}

// view icon icon {
//   size: \"60\"
//   roundedCorner: true
// }

widget title {
table: contacts:
view icon icon {
size: \"60\"
roundedCorner: true
}

layout column {
tile value logo {
value: \"/isa/BDJPFRDMEYBPBKLVADAYFQCDAVIOEQJR/magdalenas/defaultLogo.PNG\"//\"http://is1.mzstatic.com/image/thumb/Purple71/v4/89/51/f4/8951f4f1-fd6b-fa59-38b2-191140473b9a/source/175x175bb.jpg\"
view: icon
}
}
layout column {
layout row {
tile value firstName {
value: contacts:FirstName
}
tile value lastName {
value: contacts:LastName
}
tile role {
value: contacts:ContactRole
}
}
layout row {
tile company {
value: contacts:AccountName
navigateTo: \"Account\"
}
}
}
}

widget summary {
size: large
table: contacts:
tile contactDetails cc {
title: contacts:Title
role: contacts:ContactRole
email: contacts:email
phone: contacts:Phone
industry: contacts:Industry
}

tile accountDetails cc4 {
accountOwner: contacts:AccountOwner
salesManager: contacts:SalesLeader1
region: contacts:WorldRegion
revenue: contacts:AnnualAccountValue
renewalDate: accounts:RenewalDate

}

tile metric a {
label: \"LTR\"
value: average(score(survey:Q1))
target: 9
}

tile responseRate {
label: \"Response Status\"
invites: COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")
responses: COUNT(survey:responseId,survey:status=\"complete\" OR survey:status=\"incomplete\")
}

// tile metric da {
//   label: \"Surveys\"
//   showThermometer: false
//   value: count(survey:responseid)
//   target: 0
// }

// tile casesStatus {
//   label: Cases
//   open: count(cases:CaseId)
//   overdue: 0
// }
}
widget contactSurveys {
label: \"Surveys\"
table: survey:
sortColumn: surveyDate
sortOrder: ascending
size: large
navigateTo: \"Response\"

view metric metrics {
backgroundColorFormatter: backgroundColorFormatter
valueColorFormatter: valueColorFormatter
fontSize:small
//roundCorners:true
}

column value status {
label: \"Status\"
value: survey:status
}


column date surveyDate {
label: \"Submitted\"
value: survey:interview_start
format: date11
}

column metric ltr {
label: \"LTR\"
value: average(score(survey:Q1))
view: metrics
target: 9
align: center
}

// column value case2 {
//   label: \"Cases\"
//   value: COUNT(cases:CaseId)
//   align: center
//   format: responsesFormat
// }

column value s5 {
label: \"Email\"
value: survey:email
}

column value comments {
label: \"Comments\"
value: survey:Q2
}
}
}

page account \"Response\"  {
modal: true
widget contactSurveyResponse {
view title defaultSurveyResponseTitle {
}

size: medium
surveyResponseTitle {
// contactName: (contacts:FirstName + \" \") + contacts:LastName
// surveyName: \"Relationship Survey\"
tile title rt {
value: (contacts:FirstName + \" \") + contacts:LastName + \" - Relationship Survey\"
surveyName: \"Relationship Survey\"
//sprint 65 NSA need navigation via < tile link >to Contact and Account
view: defaultSurveyResponseTitle
}
}
summary {
rows: 4
tile list list1 {
item value {
value: survey:FirstMailedDate
label: \"Received\"
format: date12
}
item value {
value: survey:status
label: \"Status\"
}
item email {
value: survey:interview_start
label: \"Interview Start\"
}
item value {
value: survey:interview_end
label: \"Interview End\"
}
}
tile list list2 {
item value {
value:\"Relationship Survey\"
label: \"Source\"
}
item email {
value: survey:responseid
label: \"Response ID\"
}
item value {
value: contacts:contactid
label: \"Respondent ID\"
}
}
} // end of summary

tab {
label: \"Responses\"
tile list {
label:\" \"
item comment {
label: \"First Name\"
value: contacts:FirstName
}
item comment {
label: \"Last Name\"
value: contacts:LastName
}
item comment {
label:\"Company name\"
value: accounts:AccountName
navigateTo: \"Account\"
}
item comment {
label:\"Title\"
value: contacts:Title
}
item comment {
label:\"Role\"
value: contacts:ContactRole
}
view : defaulViewForListTile
}
tile list {
label: \"Key Metrics\"
item bar {
label: \"Likelihood to Recommend\"
value: average(score(survey:Q1))
}
item comment {
label: \"Comment\"
value: survey:Q2
}

item bar {
label: \"Team Satisfaction\"
value: average(score(survey:Q4))
}
item comment {
label:\"Comment\"
value: survey:Q6
}
view : defaulViewForListTile
}
tile list {
label: \"Product Satisfaction\"
item bar {
label: \"Technology\"
value: average(score(survey:Q7))
}
item comment {
label:\"Comment\"
value: survey:Q8
}
item bar {
label: \"Product is scalable\"
value: average(score(survey:Q9.1))
}
item bar {
label:\"Product is easy to use\"
value: average(score(survey:Q9.2))
}
item bar {
label:\"Product delivers results\"
value: average(score(survey:Q9.3))
}

view : defaulViewForListTile
}
tile list {
label: \"Business benefit\"
item bar {
label: \"Provide Added Value\"
value: average(score(survey:Q3))
}
item bar {
label: \"Support business needs\"
value: average(score(survey:Q12))
}
item comment {
label:\"Areas of Improvement\"
value: survey:Q11
}
view : defaulViewForListTile
}
}
}
}

page account \"AOResponse\"  {
modal:true
mainTable: healthCheck:
widget contactSurveyResponse internal {
view title defaultSurveyResponseTitle {
}

size: medium
surveyResponseTitle {

tile title rt {
value: accounts:AccountOwner
surveyName: \"Internal View Survey\"
//sprint 67 NSA need navigation via < tile link >to Contact and Account
view: defaultSurveyResponseTitle
}
}
summary {
rows: 4
tile list list1 {
item value {
value:healthCheck:UploadedDate
label: \"Received\"
format: date12
}
item value {
value: healthCheck:status
label: \"Status\"
}
item email {
value: healthCheck:interview_start
label: \"Interview Start\"
}
item value {
value: healthCheck:interview_end
label: \"Interview End\"
}
}
tile list list2 {
item value {
value:\"Team Check Survey\"
label: \"Source\"
}
}
}

tab {
label: \"Responses\"
tile list {
label:\" \"
item comment {
label: \"Owner Name\"
value: accounts:AccountOwner
}

item comment {
label:\"Company name\"
value: accounts:AccountName
}

item comment {
label:\"Sales region\"
value: accounts:SalesRegion
}
view : defaulViewForListTile
}
tile list {
label: \"Key Metrics\"
item bar {
label: \"Likely to Recommend\"
value: average(score(survey:Q1))
}

item bar {
label: \"Likely to Renew\"
value: average(score(healthCheck:Q2))
}
item bar {
label: \"Growth Potential\"
value: average(score(healthCheck:Q5))
}
// item comment {
//   label:\"Comment\"
//   value: survey:Q6
// }
view : defaulViewForListTile
}
tile list {
label: \"Comments\"


item comment {
label:\"How to keep the customer\"
value: healthCheck:Q4
}

item comment {
label:\"How to increase spend\"
value: healthCheck:Q6
}
view : defaulViewForListTile
}
}
}
}


";
