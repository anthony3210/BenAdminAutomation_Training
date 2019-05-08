<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>This is the API POST to create a client.</description>
   <name>CreateClient</name>
   <tag></tag>
   <elementGuidId>19b7ff30-7563-4aca-9479-e7beffad4bed</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;parentClientUUID\&quot;:\&quot;ec7052a6-7b02-4110-b0f9-288284956dc2\&quot;,\n\&quot;legalClientName\&quot;:\&quot;${legalClientName}\&quot;,\n\&quot;clientName\&quot;:\&quot;${clientName}\&quot;,\n\&quot;sicCode\&quot;:\&quot;1234\&quot;,\n\&quot;clientEffectiveDate\&quot;:\&quot;2019-01-01\&quot;,\n\&quot;clientTerminationDate\&quot;:\&quot;2020-12-31\&quot;,\n\&quot;streetAddressUUID\&quot;:\&quot;11d231c2-5f06-4387-ab4d-09d5ffbfe23c\&quot;,\n\&quot;billingAddressUUID\&quot;:\&quot;11d231c2-5f06-4387-ab4d-09d5ffbfe23c\&quot;,\n\&quot;subDomain\&quot;:\&quot;sub\&quot;,\n\&quot;memberUniqueIdentifier\&quot;:\&quot;member_uuid\&quot;,\n\&quot;memberEmailCommunication\&quot;:\&quot;true\&quot;,\n\&quot;inviteKey\&quot;:\&quot;123\&quot;,\n\&quot;billingContactEmail\&quot;:\&quot;123@gmail.com\&quot;,\n\&quot;configurationDemo\&quot;:\&quot;false\&quot;,\n\&quot;configurationInActive\&quot;:\&quot;false\&quot;,\n\&quot;configurationTemplate\&quot;:\&quot;false\&quot;,\n\&quot;primaryContactName\&quot;:\&quot;Test\&quot;,\n\&quot;primaryContactPhone\&quot;:\&quot;3125551212\&quot;,\n\&quot;primaryContactEmail\&quot;:\&quot;${primaryContactEmail}\&quot;,\n\&quot;billingContactName\&quot;:\&quot;Test\&quot;,\n\&quot;billingContactPhone\&quot;:\&quot;3125551212\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.benadmin.qa.maestroedgy.com/api/clientperson/v1/client/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Jimmy'</defaultValue>
      <description></description>
      <id>6577ab35-3d5b-4623-9c70-9699f5c56f82</id>
      <masked>false</masked>
      <name>legalClientName</name>
   </variables>
   <variables>
      <defaultValue>'Lancer'</defaultValue>
      <description></description>
      <id>6538fa6e-3602-4b87-becb-52b0d19f1138</id>
      <masked>false</masked>
      <name>clientName</name>
   </variables>
   <variables>
      <defaultValue>'jlancertest@test.com'</defaultValue>
      <description></description>
      <id>bc0e45a9-7ce5-450d-9635-b1c260f72eba</id>
      <masked>false</masked>
      <name>primaryContactEmail</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()






</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
