mod identifier_matcher;
mod literal_matcher;
mod matcher;
mod whitespace_matcher;

use identifier_matcher::IdentifierMatcher;
use identifier_matcher::NumberMatcher;
use identifier_matcher::ReferenceMatcher;
use literal_matcher::LiteralMatcher;
use matcher::Matcher;
use whitespace_matcher::WhitespaceMatcher;
use serde_derive::{Deserialize, Serialize};

use crate::lexer::identifier_matcher::{StringMatcher, CommentsMatcher};


#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
pub enum TokenType {
  Identifier,
  EntityId,
  Reference,
  String,
  Number,
  Colon,
  Comma,
  Equal,
  LessThan,
  MoreThan,
  Percent,
  OpenBracket,
  CloseBracket,
  OpenParen,
  CloseParen,
  Plus,
  Minus,
  Hash,
  Div,
  Mul,
  Dot,
  EOL,
  Comment,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Token {
  pub start: usize,
  pub end: usize,
  pub kind: TokenType,
  pub text: Option<String>,
}

pub struct Lexer {
  matchers: Vec<(Box<dyn Matcher>, TokenType)>,
}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer {
      matchers: vec![
        (Box::new(IdentifierMatcher::new()), TokenType::Identifier),
        (Box::new(ReferenceMatcher::new()), TokenType::Reference),
        (Box::new(NumberMatcher::new()), TokenType::Number),
        (Box::new(CommentsMatcher::new()), TokenType::Comment),
        (Box::new(StringMatcher::new('"')), TokenType::String),
        (Box::new(StringMatcher::new('\'')), TokenType::String),
        (Box::new(LiteralMatcher::new(":")), TokenType::Colon),
        (Box::new(LiteralMatcher::new(",")), TokenType::Comma),
        (Box::new(LiteralMatcher::new(".")), TokenType::Dot),
        (Box::new(LiteralMatcher::new("=")), TokenType::Equal),
        (Box::new(LiteralMatcher::new("<")), TokenType::LessThan),
        (Box::new(LiteralMatcher::new(">")), TokenType::MoreThan),
        (Box::new(LiteralMatcher::new("%")), TokenType::Percent),
        (Box::new(LiteralMatcher::new("{")), TokenType::OpenBracket),
        (Box::new(LiteralMatcher::new("}")), TokenType::CloseBracket),
        (Box::new(LiteralMatcher::new("(")), TokenType::OpenParen),
        (Box::new(LiteralMatcher::new(")")), TokenType::CloseParen),
        (Box::new(LiteralMatcher::new("+")), TokenType::Plus),
        (Box::new(LiteralMatcher::new("-")), TokenType::Minus),
        (Box::new(LiteralMatcher::new("/")), TokenType::Div),
        (Box::new(LiteralMatcher::new("*")), TokenType::Mul),
        (Box::new(LiteralMatcher::new("#")), TokenType::Hash),
        (Box::new(LiteralMatcher::new("\n")), TokenType::EOL),
      ],
    }
  }

  pub fn lex(
    &self,
    input: String,
  ) -> Result<Vec<Token>, (Vec<String>, Vec<Token>)> {
    let whitespace_matcher = WhitespaceMatcher::new();
    let mut current_pos = 0;
    let end_pos = input.len();
    let mut result = vec![];
    let mut errors = vec![];
    'chars: while current_pos < end_pos {
      match whitespace_matcher.check(&input[current_pos..]) {
        Ok((0, _)) => {}
        Ok((len, _)) => current_pos += len,
        Err(_) => {}
      }
      if current_pos == end_pos {
        break 'chars;
      };

      for (m, token_type) in &self.matchers {
        match m.check(&input[current_pos..]) {
          Ok((0, _)) => {}
          Ok((pos, got_string)) => {
            result.push(Token {
              start: current_pos,
              end: current_pos + pos,
              kind: *token_type,
              text: got_string,
            });
            current_pos += pos;
            continue 'chars;
          }
          Err(_) => {}
        }
      }
      // char we could not lex - skip to next
      errors.push(format!("Unknown char at pos {} , {}", current_pos, &input[current_pos..current_pos + 1]));
      current_pos += 1;
    }
    if errors.len() > 0 {
      return Err((errors, result));
    }
    return Ok(result);
  }
}

#[test]
fn lexer_parse_whitespace() {
  let lexer = Lexer::new();
  let res = lexer.lex("        ".to_string());
  assert_eq!(Ok(vec![]), res);
}

#[test]
fn lexer_parse_literal() {
  let lexer = Lexer::new();
  let res = lexer.lex("   {     ".to_string());
  assert_eq!(
    Ok(vec![Token {
      start: 3,
      end: 4,
      kind: TokenType::OpenBracket,
      text: None,
    }]),
    res
  );
}

#[test]
fn lexer_parse_identifier() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![Token {
      start: 3,
      end: 8,
      kind: TokenType::Identifier,
      text: Some("hello".to_string()),
    }]),
    lexer.lex("   hello     ".to_string())
  );
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      },
      Token {
        start: 12,
        end: 17,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      }
    ]),
    lexer.lex("   hello    hello ".to_string())
  );
}

#[test]
fn lexer_parse_string() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![Token {
      start: 3,
      end: 9,
      kind: TokenType::String,
      text: Some("1234".to_string()),
    }]),
    lexer.lex("   '1234'     ".to_string())
  );
}

#[test]
fn lexer_parse_identifier_entityid_reference() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      },
      Token {
        start: 12,
        end: 18,
        kind: TokenType::Reference,
        text: Some("hello".to_string()),
      },
      Token {
        start: 19,
        end: 20,
        kind: TokenType::Hash,
        text: None,
      },
      Token {
        start: 20,
        end: 25,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      },
      Token {
        start: 26,
        end: 32,
        kind: TokenType::Number,
        text: Some("121234".to_string()),
      }
    ]),
    lexer.lex("   hello    @hello #hello 121234".to_string())
  );
}


#[test]
fn lexer_multiline() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      },
      Token {
        start: 8,
        end: 9,
        kind: TokenType::EOL,
        text: None,
      },
      Token {
        start: 10,
        end: 15,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      }
    ]),
    lexer.lex("   hello\n hello ".to_string())
  );
}

#[test]
fn lexer_parse_identifier_and_literal() {
  let lexer = Lexer::new();
  assert_eq!(
    Ok(vec![
      Token {
        start: 3,
        end: 8,
        kind: TokenType::Identifier,
        text: Some("hello".to_string()),
      },
      Token {
        start: 9,
        end: 10,
        kind: TokenType::OpenBracket,
        text: None,
      },
      Token {
        start: 10,
        end: 11,
        kind: TokenType::CloseBracket,
        text: None,
      }
    ]),
    lexer.lex("   hello {} ".to_string())
  );
}

#[test]
fn lexer_lots() {
  let lexer = Lexer::new();
  let lexed = lexer.lex("value: MAX(survey:Q2,survey:interview_start=max(survey:interview_start))
value: average(score(survey:Q7), @cr.currentPeriodB2b)
thresholds: #82D854 >= 100%, #FFBD5B >= 80%, #FA5263 < 80%
riskValue: IIF(average(SCORE(survey:Q1))<7,'H!',IIF(average(SCORE(survey:Q1))>8,'L',IIF(COUNT(survey:responseid)<1,'U','M')))".to_string());
  assert_eq!(lexed.is_ok(), true);
  assert_eq!(lexed.unwrap().len(), 108);
}

#[test]
fn lexer_unknown_char() {
  let lexer = Lexer::new();
  assert_eq!(
    Err((
      vec!["Unknown char at pos 9 , !".to_string()],
      vec![
        Token {
          start: 3,
          end: 8,
          kind: TokenType::Identifier,
          text: Some("hello".to_string()),
        },
        Token {
          start: 11,
          end: 16,
          kind: TokenType::Identifier,
          text: Some("hello".to_string()),
        }
      ]
    )),
    lexer.lex("   hello ! hello".to_string())
  );
}

#[test]
fn lexer_large() {
  let lexer = Lexer::new();
  let lexed = lexer.lex("
    title \"For QA testing\"

config hub {
    hub: 432
    table accounts = crmdata.ArtuAccountHierarchy
    table survey = p1027835.responseid
    table contacts = p1028592.response
    table healthCheck = p1028039.responseid
    table cases = am.case
    table respondent = p1027835.respondent
    table revenue = crmdata.Historical_Revenue

    // accounts --> Health
    relation oneToMany rel1 {
      primaryKey: accounts:AccountID
      foreignKey: healthCheck:AccountID
    }
    // accounts --> contact db
    relation oneToMany rel2 {
      primaryKey: accounts:AccountID
      foreignKey: contacts:AccountID
    }
    relation oneToMany rel3 {
       primaryKey: accounts:AccountID
       foreignKey: revenue:AccountID
    }
}

config report cr {
    logo: \"http://co-osl-tenta96.firmglobal.com/isa/BDJPFRDMEYBPBKLVADAYFQCDAVIOEQJR/confirmit_logo_28x140.png\"
palette: \"#86ABE2\",\"#4079D0\",\"#1B6600\",\"#2D9900\",\"#9CCB00\",\"#FEFE00\",\"#F9BF00\",\"#F18500\",\"#EF6300\",\"#F30000\",\"#AA0010\",\"#C0C0C0\"
formatter number formatterLTR {
numberDecimals : 1
decimalSeparator : \".\"
}
formatter number formatterRR {
numberDecimals : 0
decimalSeparator : \".\"
shortForm : true
postfix : \"%\"
}
formatter number customEmpty{
numberDecimals : 0
emptyValue: -
}
formatter number floatNumber {
numberDecimals: 3
}
formatter date dateFormat {
inputFormat: \"YYYYMM\"
formatString: \"YYYY MMMM\"
}
formatter objectProperty textPicker {
property: text
}
formatter color backgroundColor {
thresholds: #e8f8e0 >= 100%, #ffeed6 >= 80%, #fedfe2 >= 0%
}
formatter color valueColor {
thresholds: #388e3c >= 100%, #ff6d00 >= 80%, #d40000 >= 0%
}
formatter color kpiColorFormatter {
thresholds: #82D854 >= 100%, #FFBD5B >= 80%, #FA5263 < 80%
}
formatter date DDMMMYYYY {
format: \"DD MMM YYYY\"
shortForm: true
emptyValue: -
}
formatter date dateRelative {
locale:en
shortForm: false
relative:true
}
formatter text commentFormat {
useDots:true
length:68
emptyValue: -
}


completeSurv: COUNT(survey:responseid,survey:status=\"Complete\")
ltrValue: average(score(survey:Q1))
ltrTarget: 9
healthTarget: 8
riskValue: IIF(average(SCORE(survey:Q1))<7,'H!',IIF(average(SCORE(survey:Q1))>8,'L',IIF(COUNT(survey:responseid)<1,'U','M')))
riskTarget: 10
rateInvites: COUNT(respondent:respid,respondent:smtpstatus=\"messagesent\")
rateResponses: @cr.completeSurv
rateValue: (@cr.rateResponses/@cr.rateInvites)*100
casesValue: COUNT(cases:CaseId,cases:SystemStatus='Open')
fullContactName: (contacts:FirstName + \" \") + contacts:LastName
currentPeriodHealth: healthCheck:interview_start > 2016-06-22
previousPeriodHealth: healthCheck:interview_start <= 2016-06-22
currentPeriodB2b: survey:interview_start > 2016-06-22
previousPeriodB2b: survey:interview_start <= 2016-06-22
highRiskLogo: \"/isa/BDJPFRDMEYBPBKLVADAYFQCDAVIOEQJR/mch/HighRisk.PNG\"
warningLogo: \"/isa/BDJPFRDMEYBPBKLVADAYFQCDAVIOEQJR/mch/warning.PNG\"
blankLogo: \"/isa/BDJPFRDMEYBPBKLVADAYFQCDAVIOEQJR/mch/Blank.PNG\"
contactLogo: \"/isa/BDJPFRDMEYBPBKLVADAYFQCDAVIOEQJR/mch/53633418-5037-4CEB-AF68-D8616D95094B.jpg\"
}

layoutArea toolbar {
filter multiselect {
optionsFrom: survey:NPSSegment
}
filter multiselect {
optionsFrom: cases:lk_7
}
filter multiselect {
label: \"Account Rating\"
option checkbox {
label:\"Gold\"
value:accounts:TotalAccountValue > 200000
}
option checkbox {
label:\"Silver\"
value: accounts:TotalAccountValue >99999 AND accounts:TotalAccountValue <199999
}
option checkbox {
label:\"Bronze\"
value: accounts:TotalAccountValue < 100000
}
}
}

page \"Account List\" {
widget search {
layoutArea: \"header\"
source search account {
table: accounts:
value: (accounts:AccountName+\" \")+accounts:AccountID
navigateTo: \"Account\"
iconType: \"account\"
}
source search contact {
table: contacts:
value: @cr.fullContactName
navigateTo: \"Contact\"
}
}
widget accountList{
label: \"Accounts\"
size: large
table: accounts:
sortColumn: accountName
sortOrder: ascending
navigateTo: \"Account\"
hierarchy: accounts:ParentAccountID

view metricWithChange metrics {
backgroundColorFormatter: backgroundColor
valueColorFormatter: valueColor
fontSize:medium
roundCorners:true
}
view icon icon {
size: \"60\"
roundedCorner: true
}
view icon iconSmall {
size: \"20\"
roundedCorner: false
}

column value accountID {
label: \"Account ID\"
value: accounts:AccountID
}
column hierarchy accountName {
label: \"Account Name\"
value: accounts:AccountName
rowHeader: true
}
column value accountMan {
label: \"Account Owner\"
value: accounts:AccountOwner
}
column metric health11 {
label: \"Health\"
value : average(score(healthCheck:Renew), @cr.currentPeriodHealth)
previous: average(score(healthCheck:Renew), @cr.previousPeriodHealth)
target: @cr.healthTarget
format: formatterLTR
view: metrics
}
column value LTR {
label: \"LTR\"
value: @cr.ltrValue
format: formatterLTR
align: center
}
column metric osat {
label: \"OSAT\"
value: average(score(survey:Q4))
format: formatterLTR
view:metrics
target:9
}
column value total {
label: \"Revenue ($)\"
value:accounts:TotalAccountValue
format: formatterLTR
}
column value case1 {
label: \"Cases\"
value:@cr.casesValue
sortable: false
format: customEmpty
}
column value risk {
label: \"High Risk\"
value:IIF(average(SCORE(survey:Q1))<7,@cr.highRiskLogo,IIF(average(SCORE(survey:Q1))>8,@cr.blankLogo,IIF(COUNT(survey:responseid)<1,@cr.blankLogo,@cr.warningLogo)))
view: iconSmall
}
column value responses {
label: \"Responses\"
value: @cr.completeSurv
}
column value rate {
label: \"Response Rate\"
value: @cr.rateValue
format: formatterRR
}
column value noResp {
label: \"No Response\"
value: COUNT(survey:responseid)-@cr.completeSurv //COUNT(survey:responseid,survey:smtpstatus=\"Sent\")
}
column value rev2015 {
label: \"2015 rev\"
value: sum(revenue:AnnualAccountValue, revenue:year=2015)
format: formatterLTR
align: right
}
column value survCount {
label: \"Surveys\"
value: count(survey:responseid)
}
}
}
page account \"Account\" {
widget search {
layoutArea: \"header\"
source search contact {
table: contacts:
value: @cr.fullContactName
navigateTo: \"Contact\"
}
}
widget title {
table: contacts:
layout column {
layout row {
tile value {
value: \"ffsdfsdfs \"
}

tile role {
value: \"some role\"
}

}

layout row {
tile company {
value: \"some company\"
}

}

}

layout column {
tile value {
value: \"ffsdfsdfs \"
}

tile company {
value: contacts:AccountName
navigateTo: \"Account1\"
}

}

}

widget summary {
table: accounts:
size: large
tile metric {
label: \"LTR Average\"
value: @cr.ltrValue
target: @cr.ltrTarget
}
tile metric {
label: \"Health Check\"
value: average(score(healthCheck:Renew))
target: @cr.healthTarget
backgroundColorFormatter: valueColor
}
tile risk {
label: \"Renewal Risk\"
value: @cr.riskValue
target: @cr.riskTarget
min: 0
max: 10
renewal: accounts:renewalDate
revenue: accounts:TotalAccountValue
textValue : 'Risk Text'
}
tile responseRate {
invites: @cr.rateInvites
responses: @cr.rateResponses
}
tile casesStatus {
open: @cr.casesValue
overdue: 0
}
}
widget accountComments {
size: large
}
widget contactList {
label: \"Contacts\"
table: contacts:
size: large
sortColumn: name
sortOrder: descending
navigateTo: \"Contact\"

view metricWithChange metrics {
backgroundColorFormatter: backgroundColor
valueColorFormatter: valueColor
fontSize:medium
roundCorners:true
}

column value name {
label: \"Name\"
value: @cr.fullContactName
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
value: @cr.ltrValue
target:@cr.ltrTarget
format: formatterLTR
view:metrics
}
column value openCases {
label: \"Cases\"
value: @cr.casesValue
format: customEmpty
}
column value lastResponse {
label: \"Last response\"
value: max(survey:interview_end)
format:DDMMMYYYY
asign: center
}
column value comments {
label: \"Comments\"
value: MAX(survey:Q2,survey:interview_start=max(survey:interview_start))
format: commentFormat
}
column value survCount {
label: \"Surveys\"
value: count(survey:responseid)
}
}
widget accountCases {
label: \"Cases\"
table: cases:
size: large
sortColumn: dateCreated
sortOrder: descending

view link openLink {
label: \"Open link\"
}

column value dateCreated {
label: \"Date\"
value: cases:DateCreated
asign: center
format: dateRelative
}
column value Sev {
label: \"Severity\"
value: cases:lk_8
format: textPicker
}
column value caseSev {
label: \"Status\"
value: cases:lk_7
format: textPicker
}
column value caseCat {
label: \"Category\"
value: cases:Workflow
format: textPicker
}
column value res {
label: \"Resolution\"
value: cases:lk_16
format: textPicker
}
column value f {
label: \"CaseLink \"
value: cases:CaseLink
view: openLink
}
}
}
page contact \"Contact\" {
widget search {
layoutArea: \"header\"
source search account {
table: accounts:
value: (accounts:AccountName+\" \")+accounts:AccountID
navigateTo: \"Account\"
iconType: \"account\"
}
}

widget summary {
size: large
table: contacts:
tile contactDetails cc {
role: contacts:ContactRole
email: contacts:email
phone: contacts:Phone
title: contacts:AccountName
industry: contacts:ContactId
}
tile accountDetails cc4 {
accountOwner: accounts:AccountOwner
salesManager: accounts:SalesLeader1
region: accounts:WorldRegion
revenue: accounts:AnnualAccountValue
renewalDate: accounts:RenewalDate
}
tile metric {
label: \"LTR Average\"
value: @cr.ltrValue
target: @cr.ltrTarget
}
tile surveyResponses da {
label: \"Surveys\"
total: count(survey:responseid)
completed:@cr.completeSurv
}
}
widget accountCases {
label: \"Cases\"
table: cases:
size: large
sortColumn: dateCreated
sortOrder: descending

view link openLink {
label: \"Open link\"
}

column value dateCreated {
label: \"Date\"
value: cases:DateCreated
asign: center
format:dateRelative
}
column value Sev {
label: \"Severity\"
value: cases:lk_8
format: textPicker
}
column value caseSev {
label: \"Status\"
value: cases:lk_7
format: textPicker
}
column value caseCat {
label: \"Category\"
value: cases:Workflow
format: textPicker
}
column value res {
label: \"Resolution\"
value: cases:lk_16
format: textPicker
}
column value f {
label: \"CaseLink \"
value: cases:CaseLink
view: openLink
}
}
widget contactSurveys {
label: \"Surveys\"
table: survey:
sortColumn: contactID
sortOrder: descending
size: large

column value contactID {
label: \"Contact ID\"
value: contacts:ContactID
}
column value contactStatus {
label: \"Contact Status\"
value: survey:status
format: textPicker
}
column value interviewStart {
label: \"Interview Start\"
value: survey:interview_start
format:DDMMMYYYY
}
column value responseId {
label: \"ResponseId\"
value: survey:responseid
}
column value comments {
label: \"Comments\"
value: survey:Q8
}
}
}
page \"Overview\" {
widget kpi {
label: \"NPS Score\"
size: small
tile kpi {
label:\"NPS\"
value: NPS(survey:Q1)*100
target: 50
min: -100
max: 100
format:formatterLTR
targetFormat:formatterLTR
gaugeColorFormat:kpiColorFormatter  // valueColor
tile value {
label: \"Responses\"
value: count(survey:Q1,@cr.currentPeriodB2b)
max: count(survey:responseid, @cr.currentPeriodB2b)
format: integer
}
tile value {
label: \"Yearly change\"
value: average(score(survey:Q1),@cr.currentPeriodB2b)-average(score(survey:Q1),@cr.previousPeriodB2b)
format:formatterLTR
}
}
}
widget kpi {
label: \"Overall Satisfaction\"
size: small
tile kpi {
label:\"OSAT\"
value: average(score(survey:Q4))
target: 9
min: 0
max: 10
format:formatterLTR
targetFormat:formatterLTR
gaugeColorFormat: kpiColorFormatter
tile value {
label: \"Responses\"
value:count(survey:responseid,survey:status=\"Complete\" AND @cr.currentPeriodB2b)
max: count(survey:responseid, @cr.currentPeriodB2b)
format: integer
}
tile value {
label: \"Yearly change\"
value: average(score(survey:Q4),@cr.currentPeriodB2b) -average(score(survey:Q4),@cr.previousPeriodB2b)
format:formatterLTR
}
}
}
widget portfolioBreakdown {
label: \"Month vs Satisfaction\"
size: medium
category: CalendarMonth(survey:interview_start)
segment: survey:Q1
value: count(survey:responseId)
format: floatNumber
categoryFormat: dateFormat
palette: @cr.palette
}
widget recentResponses {
table: survey:
size: medium
take: 10
label: \"Title for Recent responses\"
view comment commentView {
lines: 2
}
view metricWithChange metrics {
backgroundColorFormatter: backgroundColor
valueColorFormatter: valueColor
fontSize:large
roundCorners:true
}
column response {
footer: survey:interview_end
footerFormat: dateRelative
header: (accounts:AccountName + \" - \") + ((contacts:FirstName + \" \") + contacts:LastName)
comment: survey:Q2
view: commentView
}
column value ltr {
label: \"LTR\"
value: average(score(survey:Q1))
}
column metric product {
label: \"Product\"
value: average(score(survey:Q7), @cr.currentPeriodB2b)
previous: average(score(survey:Q7), @cr.previousPeriodB2b)
target: 9
view:metrics
format: formatterLTR
}
}
}
    ".to_string());
  assert_eq!(lexed.is_ok(), true);
  assert_eq!(lexed.unwrap().len(), 2944);
//  assert_eq!(lexed.unwrap().len(), 108);
}
